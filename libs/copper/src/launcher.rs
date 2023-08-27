use derive_builder::Builder;
use error_stack::{IntoReport, Result, ResultExt};
use std::{
    error::Error as ErrorTrait,
    fmt::{self, Display, Formatter},
    path::PathBuf,
    process::{ExitStatus, Stdio},
    sync::Arc,
};
use tokio::{
    io::BufReader,
    process::{ChildStderr, ChildStdout, Command},
    sync::mpsc::{self, UnboundedReceiver, UnboundedSender},
    task::JoinHandle,
};
use tracing::{debug, error};

#[cfg(target_os = "windows")]
use winsafe::IsWindows10OrGreater;

use crate::{
    assets::{
        asset_index::{AssetDownloader, Assets, Object},
        client::{self, Artifact, ClassDownloader, DownloadClass, Library, LibraryDownloader},
    },
    auth::{
        structs::{MinecraftProfile, MinecraftToken},
        MSauth,
    },
    downloader::{DownloadError, DownloadMessage, Downloader as DownloaderTrait},
    parser::{JvmArgs, MinecraftArgs},
};

#[derive(Debug, Clone)]
pub struct AuthenticationDetails {
    pub authenticator: MSauth,
    pub auth_details: MinecraftToken,
    pub minecraft_profile: MinecraftProfile,
    pub is_demo_user: bool,
}

#[derive(Debug, Clone)]
pub struct CustomResolution {
    pub width: i32,
    pub height: i32,
}

#[derive(Debug, Clone)]
pub struct RamSize {
    pub min: String,
    pub max: String,
}

pub struct GameOutput {
    pub stdout: BufReader<ChildStdout>,
    pub stderr: BufReader<ChildStderr>,
    pub exit_handle: JoinHandle<Option<ExitStatus>>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Quickplay {
    /// Singleplayer quickplay. Inner value is a world name
    Singleplayer(String),
    /// Multiplayer quickplay. Inner value is a server address
    Multiplayer(String),
    /// Realms quickplay. Inner value is a realm ID
    Realms(String),
}

impl Quickplay {
    #[must_use]
    pub const fn is_singleplayer(&self) -> bool {
        matches!(self, Self::Singleplayer(_))
    }

    #[must_use]
    pub const fn is_multiplayer(&self) -> bool {
        matches!(self, Self::Multiplayer(_))
    }

    #[must_use]
    pub const fn is_realms(&self) -> bool {
        matches!(self, Self::Realms(_))
    }
}

#[derive(Debug, Builder)]
#[builder(setter(into))]
pub struct Launcher {
    /// The authentication details (username, uuid, access token, xbox uid, etc)
    authentication_details: AuthenticationDetails,
    /// A custom resolution to use instead of the default
    custom_resolution: Option<CustomResolution>,
    /// The path to the client.jar file
    jar_path: PathBuf,
    /// The root .minecraft folder
    game_directory: PathBuf,
    /// The assets directory, this is the root of the assets folder
    assets_directory: PathBuf,
    /// The libraries directory, this is the root of the libraries folder
    libraries_directory: PathBuf,
    /// The path to <version>.json
    version_manifest_path: PathBuf,
    /// is this version a snapshot
    is_snapshot: bool,
    /// The version name
    version_name: String,
    /// The min/max amount of ram to use
    ram_size: RamSize,
    /// The path to javaw.exe
    java_path: PathBuf,
    /// The launcher name (e.g glowsquid)
    launcher_name: String,
    /// The launcher version
    launcher_version: String,
    /// If you want to launch with quickplay
    quickplay: Option<Quickplay>,
    /// The reqwest client
    http_client: reqwest::Client,
    /// The manifest the launcher will use
    manifest: client::Manifest,
}

#[derive(Debug)]
pub enum Error {
    CannotGetStdout,
    CannotGetStderr,
    ProcessError,
    AuthError,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::CannotGetStdout => "Cannot get stdout",
            Self::CannotGetStderr => "Cannot get stderr",
            Self::ProcessError => "Process error",
            Self::AuthError => "Authentication Error",
        })
    }
}

impl ErrorTrait for Error {}

/// High-level API
impl Launcher {
    /// Launches the game, assuming all the required files are downloaded
    ///
    /// # Errors
    /// If the process cannot be spawned, or the stdout/stderr cannot be read
    #[tracing::instrument(skip(self))]
    pub async fn launch(&mut self) -> Result<GameOutput, Error> {
        debug!("Launching game");

        if self.authentication_details.auth_details.is_expired() {
            self.authentication_details
                .auth_details
                .refresh(&self.authentication_details.authenticator)
                .await
                .change_context(Error::AuthError)?;
        }

        let mut game_args = MinecraftArgs::new(self, &self.manifest).parse_minecraft_args();
        let mut jvm_args = JvmArgs::new(self, &self.manifest).parse_jvm_args();

        debug!("Game args: {:?}", game_args);
        debug!("JVM args: {:?}", jvm_args);

        let main_class = self.manifest.main_class();

        debug!("Main class: {}", main_class);

        let mut process = Command::new(self.java_path.clone())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .args(&mut jvm_args)
            .arg(main_class)
            .args(&mut game_args)
            .spawn()
            .into_report()
            .change_context(Error::ProcessError)?;

        let stdout = process.stdout.take().ok_or(Error::CannotGetStdout)?;

        let stderr = process.stderr.take().ok_or(Error::CannotGetStderr)?;

        let out_reader = BufReader::new(stdout);
        let err_reader = BufReader::new(stderr);

        let exit = tokio::spawn(async move { process.wait().await.ok() });

        Ok(GameOutput {
            stderr: err_reader,
            stdout: out_reader,
            exit_handle: exit,
        })
    }
}

#[derive(Debug)]
pub enum DownloadItem {
    Asset(Object),
    Library(Artifact),
    Client(DownloadClass),
}

#[derive(Debug, Clone)]
pub struct Downloader {
    /// The sender for the download channel
    sender: Option<UnboundedSender<DownloadMessage<DownloadItem>>>,

    asset_downloader: AssetDownloader,
    library_downloader: LibraryDownloader,
    client_download: ClassDownloader,
}

impl Downloader {
    #[must_use]
    pub fn new(
        launcher: &Launcher,
        assets: Assets,
        libraries: Vec<Library>,
        max_concurrent_downloads: usize,
    ) -> Self {
        let asset_downloader = AssetDownloader::new(
            assets,
            launcher.assets_directory.clone(),
            launcher.http_client.clone(),
            max_concurrent_downloads / 2,
        );

        let library_downloader = LibraryDownloader::new(
            libraries,
            launcher.libraries_directory.clone(),
            launcher.http_client.clone(),
            max_concurrent_downloads / 2,
        );

        let class_downloader = ClassDownloader::new(
            launcher.http_client.clone(),
            launcher.manifest.downloads().client().clone(),
            launcher.jar_path.clone(),
        );

        Self {
            sender: None,
            asset_downloader,
            library_downloader,
            client_download: class_downloader,
        }
    }
}

/// Downloads _everything_ needed to launch the game
#[async_trait::async_trait]
impl DownloaderTrait for Downloader {
    type DownloadItem = DownloadItem;

    #[allow(clippy::too_many_lines)]
    /// # Panics
    /// Panics if the sender is not set, which shouldn't ever happen.
    fn create_channel(&mut self) -> UnboundedReceiver<DownloadMessage<Self::DownloadItem>> {
        let (sender, reciever) = mpsc::unbounded_channel();
        self.sender = Some(sender);

        // hook up the asset downloader
        let mut asset_downloader_channel = self.asset_downloader.create_channel();
        let asset_sender = self.sender.clone().unwrap();

        tokio::spawn(async move {
            while let Some(message) = asset_downloader_channel.recv().await {
                match message {
                    DownloadMessage::Downloaded(object) => {
                        debug!("Asset downloader object {} downloaded", object.hash());
                        let hash = object.hash().to_owned();

                        if let Err(e) = asset_sender
                            .send(DownloadMessage::Downloaded(DownloadItem::Asset(object)))
                        {
                            error!(
                                "Asset downloader failed to send object {} downloaded: {}",
                                hash, e
                            );
                        }
                    }
                    DownloadMessage::DownloadProgress(object, download_progress) => {
                        debug!(
                            "Asset downloader object {} progress: {}",
                            object.hash(),
                            download_progress
                        );
                        let hash = object.hash().to_owned();

                        if let Err(e) = asset_sender.send(DownloadMessage::DownloadProgress(
                            DownloadItem::Asset(object),
                            download_progress,
                        )) {
                            error!(
                                "Asset downloader failed to send object {} progress: {}",
                                hash, e
                            );
                        }
                    }
                    DownloadMessage::DownloadedAll => {
                        debug!("Asset downloader finished downloading all assets");
                    }
                }
            }
        });

        // hook up the library downloader
        let mut library_downloader_channel = self.library_downloader.create_channel();
        let library_sender = self.sender.clone().unwrap();

        tokio::spawn(async move {
            while let Some(message) = library_downloader_channel.recv().await {
                match message {
                    DownloadMessage::Downloaded(object) => {
                        debug!("Library downloader object {} downloaded", object.sha1());
                        let hash = object.sha1().to_owned();

                        if let Err(e) = library_sender
                            .send(DownloadMessage::Downloaded(DownloadItem::Library(object)))
                        {
                            error!(
                                "Library downloader failed to send object {} downloaded: {}",
                                hash, e
                            );
                        }
                    }
                    DownloadMessage::DownloadProgress(object, download_progress) => {
                        debug!(
                            "Library downloader object {} progress: {}",
                            object.sha1(),
                            download_progress
                        );
                        let hash = object.sha1().to_owned();

                        if let Err(e) = library_sender.send(DownloadMessage::DownloadProgress(
                            DownloadItem::Library(object),
                            download_progress,
                        )) {
                            error!(
                                "Library downloader failed to send object {} progress: {}",
                                hash, e
                            );
                        }
                    }
                    DownloadMessage::DownloadedAll => {
                        debug!("Library downloader finished downloading all assets");
                    }
                }
            }
        });

        // hook up the class downloader
        let mut class_downloader_channel = self.client_download.create_channel();
        let class_sender = self.sender.clone().unwrap();

        tokio::spawn(async move {
            while let Some(message) = class_downloader_channel.recv().await {
                match message {
                    DownloadMessage::Downloaded(object) => {
                        debug!("Client downloaded");

                        if let Err(e) = class_sender
                            .send(DownloadMessage::Downloaded(DownloadItem::Client(object)))
                        {
                            error!("Client downloader failed to send downloaded: {}", e);
                        }
                    }
                    DownloadMessage::DownloadProgress(object, download_progress) => {
                        debug!("Client progress: {}", download_progress);

                        if let Err(e) = class_sender.send(DownloadMessage::DownloadProgress(
                            DownloadItem::Client(object),
                            download_progress,
                        )) {
                            error!("Client downloader failed to send progress: {}", e);
                        }
                    }
                    DownloadMessage::DownloadedAll => {
                        debug!("Client downloader finished downloading client");
                    }
                }
            }
        });

        reciever
    }

    async fn download(&self, item: Self::DownloadItem) -> Result<(), DownloadError> {
        match item {
            DownloadItem::Asset(object) => self.asset_downloader.download(object).await,
            DownloadItem::Library(library) => self.library_downloader.download(library).await,
            DownloadItem::Client(client) => self.client_download.download(client).await,
        }
    }

    async fn download_all(self: Arc<Self>) -> Result<(), DownloadError> {
        let asset_downloader = Arc::new(self.asset_downloader.clone());
        let library_downloader = Arc::new(self.library_downloader.clone());
        let client_downloader = Arc::new(self.client_download.clone());

        let assets = tokio::spawn(async move { asset_downloader.download_all().await });
        let libraries = tokio::spawn(async move { library_downloader.download_all().await });
        let client = tokio::spawn(async move { client_downloader.download_all().await });

        // wait for both to finish
        let (assets, libraries, client) = tokio::try_join!(assets, libraries, client)
            .into_report()
            .change_context(DownloadError::JoinError)?;

        assets?;
        libraries?;
        client?;

        self.sender
            .as_ref()
            .unwrap()
            .send(DownloadMessage::DownloadedAll)
            .into_report()
            .change_context(DownloadError::ChannelError)?;

        Ok(())
    }
}

/// Getter methods
impl Launcher {
    #[must_use]
    pub const fn authentication_details(&self) -> &AuthenticationDetails {
        &self.authentication_details
    }

    #[must_use]
    pub const fn custom_resolution(&self) -> Option<&CustomResolution> {
        self.custom_resolution.as_ref()
    }

    #[must_use]
    pub const fn jar_path(&self) -> &PathBuf {
        &self.jar_path
    }

    #[must_use]
    pub const fn game_directory(&self) -> &PathBuf {
        &self.game_directory
    }

    #[must_use]
    pub const fn assets_directory(&self) -> &PathBuf {
        &self.assets_directory
    }

    #[must_use]
    pub const fn libraries_directory(&self) -> &PathBuf {
        &self.libraries_directory
    }

    #[must_use]
    pub const fn version_manifest_path(&self) -> &PathBuf {
        &self.version_manifest_path
    }

    #[must_use]
    pub const fn is_snapshot(&self) -> bool {
        self.is_snapshot
    }

    #[must_use]
    pub fn version_name(&self) -> &str {
        self.version_name.as_ref()
    }

    #[must_use]
    pub const fn ram_size(&self) -> &RamSize {
        &self.ram_size
    }

    #[must_use]
    pub const fn java_path(&self) -> &PathBuf {
        &self.java_path
    }

    #[must_use]
    pub fn launcher_name(&self) -> &str {
        self.launcher_name.as_ref()
    }

    #[must_use]
    pub fn launcher_version(&self) -> &str {
        self.launcher_version.as_ref()
    }

    #[must_use]
    pub const fn quickplay(&self) -> Option<&Quickplay> {
        self.quickplay.as_ref()
    }
}
