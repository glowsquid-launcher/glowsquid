use std::{
    cmp::min,
    error::Error,
    fmt::Display,
    path::{Path, PathBuf},
    sync::Arc,
};

use error_stack::{IntoReport, Result as ErrorStackResult, ResultExt};
use futures::{stream, StreamExt, TryStreamExt};
use serde::{Deserialize, Serialize};
use serde_json::to_string;
use tokio::{
    fs,
    io::AsyncWriteExt,
    sync::mpsc::{self, UnboundedReceiver, UnboundedSender},
};
use tracing::debug;

use crate::downloader::{DownloadError, DownloadMessage, Downloader};

use super::asset_index::Assets;

#[derive(Debug)]
pub enum SaveError {
    SerializeError,
    IOError,
}

impl Display for SaveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SerializeError => write!(f, "Failed to serialize manifest to JSON"),
            Self::IOError => write!(f, "Failed during IO task"),
        }
    }
}

impl Error for SaveError {}

/// High level API
impl Manifest {
    /// Saves the manifest to disk.
    ///
    /// # Errors
    /// Returns a [`SaveError`] if the manifest could not be serialized or if an IO error occurred.
    #[tracing::instrument]
    pub async fn save_to_disk(&self, path: &Path) -> error_stack::Result<(), SaveError> {
        debug!("Saving manifest to disk");
        debug!("Serializing manifest to JSON");
        let value = to_string(self)
            .into_report()
            .change_context(SaveError::SerializeError)?;

        let directory = path.parent().ok_or(SaveError::IOError).into_report()?;

        if !directory.exists() {
            debug!("Creating directory {}", directory.display());
            fs::create_dir_all(directory)
                .await
                .into_report()
                .change_context(SaveError::IOError)?;
        }

        debug!("Writing manifest to {}", path.display());
        fs::write(path, value)
            .await
            .into_report()
            .change_context(SaveError::IOError)
    }
}

/// Getter methods
impl Manifest {
    #[must_use]
    pub const fn get_java_version(&self) -> u8 {
        match &self.java_version {
            Some(m) => m.major_version,
            None => 8,
        }
    }

    #[must_use]
    pub fn libraries(&self) -> &[Library] {
        self.libraries.as_ref()
    }

    #[must_use]
    pub const fn downloads(&self) -> &Downloads {
        &self.downloads
    }
}

// Thank you quicktype, very cool :ferrisBased:

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Manifest {
    asset_index: AssetIndex,
    assets: String,
    downloads: Downloads,
    id: String,
    libraries: Vec<Library>,
    main_class: String,
    /// Used before 1.13
    minecraft_arguments: Option<String>,
    arguments: Option<Arguments>,
    minimum_launcher_version: i64,
    java_version: Option<JavaVersion>,
    release_time: String,
    time: String,
    #[serde(rename = "type")]
    manifest_type: Type,
    logging: Option<Logging>,
    compliance_level: Option<u8>,
    /// What to merge from. During a merge, it will take the current manifest and merge this manifest into it
    inherits_from: Option<PathBuf>,
}

impl Manifest {
    /// Gets the arguments for a manifest
    ///
    /// # Panics
    /// Panics if the manifest does not any types of arguments. This should never happen in a valid
    /// manifest
    #[must_use]
    pub fn get_arguments(&self) -> Args {
        self.arguments.as_ref().map_or_else(
            || {
                Args::MinecraftArguments(
                    self.minecraft_arguments
                        .as_ref()
                        .expect("Minecraft arguments to exist when arguments are not present"),
                )
            },
            Args::Arguments,
        )
    }

    /// Gets the asset index for a manifest
    #[must_use]
    pub const fn asset_index(&self) -> &AssetIndex {
        &self.asset_index
    }

    /// Gets the main class for a manifest
    #[must_use]
    pub fn main_class(&self) -> &str {
        &self.main_class
    }
}

pub enum Args<'a> {
    MinecraftArguments(&'a str),
    Arguments(&'a Arguments),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum Type {
    Release,
    Snapshot,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AssetIndex {
    id: String,
    sha1: String,
    size: i64,
    total_size: i64,
    url: String,
}

impl AssetIndex {
    /// Downloads the asset index
    ///
    /// # Errors
    /// Returns a [`reqwest::Error`] if the asset index could not be downloaded or parsed.
    pub async fn download(&self) -> Result<Assets, reqwest::Error> {
        reqwest::get(&self.url).await?.json().await
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Downloads {
    client: DownloadClass,
    server: DownloadClass,
    client_mappings: Option<Mappings>,
    server_mappings: Option<Mappings>,
    /// Only present in version 1 of the manifest it seems
    windows_server: Option<DownloadClass>,
}

impl Downloads {
    #[must_use]
    pub const fn client(&self) -> &DownloadClass {
        &self.client
    }

    #[must_use]
    pub const fn server(&self) -> &DownloadClass {
        &self.server
    }

    #[must_use]
    pub const fn client_mappings(&self) -> Option<&Mappings> {
        self.client_mappings.as_ref()
    }

    #[must_use]
    pub const fn server_mappings(&self) -> Option<&Mappings> {
        self.server_mappings.as_ref()
    }

    #[must_use]
    pub const fn windows_server(&self) -> Option<&DownloadClass> {
        self.windows_server.as_ref()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DownloadClass {
    sha1: String,
    size: u64,
    url: String,
}

impl DownloadClass {
    #[must_use]
    pub fn sha1(&self) -> &str {
        &self.sha1
    }

    #[must_use]
    pub const fn size(&self) -> u64 {
        self.size
    }
}

#[derive(Debug, Clone)]
pub struct ClassDownloader {
    client: reqwest::Client,

    class: DownloadClass,
    path: PathBuf,
    sender: Option<UnboundedSender<DownloadMessage<DownloadClass>>>,
}

impl ClassDownloader {
    #[must_use]
    pub const fn new(client: reqwest::Client, class: DownloadClass, path: PathBuf) -> Self {
        Self {
            client,
            class,
            path,
            sender: None,
        }
    }
}

#[async_trait::async_trait]
impl Downloader for ClassDownloader {
    type DownloadItem = DownloadClass;

    fn create_channel(&mut self) -> UnboundedReceiver<DownloadMessage<Self::DownloadItem>> {
        let (sender, receiver) = mpsc::unbounded_channel();
        self.sender = Some(sender);
        receiver
    }

    async fn download(&self, item: Self::DownloadItem) -> ErrorStackResult<(), DownloadError> {
        if self
            .path
            .try_exists()
            .into_report()
            .change_context(DownloadError::IoError)?
        {
            return Ok(());
        }

        let mut response = self
            .client
            .get(&item.url)
            .send()
            .await
            .into_report()
            .change_context(DownloadError::ReqwestError)?
            .bytes_stream();

        let parent_dir = self
            .path
            .parent()
            .ok_or(DownloadError::IoError)
            .into_report()?;

        tokio::fs::create_dir_all(parent_dir)
            .await
            .into_report()
            .change_context(DownloadError::IoError)?;

        let mut file = tokio::fs::File::create(&self.path)
            .await
            .into_report()
            .change_context(DownloadError::IoError)?;

        let mut downloaded: u64 = 0;

        while let Some(item) = response.next().await {
            let item = item
                .into_report()
                .change_context(DownloadError::ReqwestError)?;

            file.write_all(&item)
                .await
                .into_report()
                .change_context(DownloadError::IoError)?;

            let new = min(downloaded + (item.len() as u64), self.class.size);
            downloaded = new;

            self.sender
                .as_ref()
                .ok_or(DownloadError::ChannelError)?
                .send(DownloadMessage::DownloadProgress(self.class.clone(), new))
                .into_report()
                .change_context(DownloadError::ChannelError)?;
        }

        self.sender
            .as_ref()
            .ok_or(DownloadError::ChannelError)?
            .send(DownloadMessage::Downloaded(self.class.clone()))
            .into_report()
            .change_context(DownloadError::ChannelError)
    }

    async fn download_all(self: Arc<Self>) -> ErrorStackResult<(), DownloadError> {
        self.download(self.class.clone()).await?;

        self.sender
            .as_ref()
            .ok_or(DownloadError::ChannelError)?
            .send(DownloadMessage::DownloadedAll)
            .into_report()
            .change_context(DownloadError::ChannelError)?;

        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Artifact {
    path: String,
    sha1: String,
    size: u64,
    url: String,
}

impl Artifact {
    #[must_use]
    pub fn path(&self) -> &str {
        self.path.as_ref()
    }

    #[must_use]
    pub fn sha1(&self) -> &str {
        self.sha1.as_ref()
    }

    #[must_use]
    pub const fn size(&self) -> u64 {
        self.size
    }

    /// Downloads the artifact to the given path, appending the path to the given path.
    ///
    /// If there is no artifact or it's already downloaded, this function will do nothing.
    ///
    /// # Errors
    /// This function will return an error if the artifact is not downloaded successfully.
    pub async fn download(
        &self,
        library_path: PathBuf,
        client: &reqwest::Client,
        sender: &UnboundedSender<DownloadMessage<Self>>,
    ) -> ErrorStackResult<(), DownloadError> {
        let path = library_path.join(self.path());

        if path
            .try_exists()
            .into_report()
            .change_context(DownloadError::IoError)?
        {
            return Ok(());
        }

        let mut response = client
            .get(&self.url)
            .send()
            .await
            .into_report()
            .change_context(DownloadError::ReqwestError)?
            .bytes_stream();

        let parent_dir = path.parent().ok_or(DownloadError::IoError).into_report()?;

        tokio::fs::create_dir_all(parent_dir)
            .await
            .into_report()
            .change_context(DownloadError::IoError)?;

        let mut file = tokio::fs::File::create(path)
            .await
            .into_report()
            .change_context(DownloadError::IoError)?;

        let mut downloaded: u64 = 0;

        while let Some(item) = response.next().await {
            let item = item
                .into_report()
                .change_context(DownloadError::ReqwestError)?;

            file.write_all(&item)
                .await
                .into_report()
                .change_context(DownloadError::IoError)?;

            let new = min(downloaded + (item.len() as u64), self.size);
            downloaded = new;

            sender
                .send(DownloadMessage::DownloadProgress(self.clone(), new))
                .into_report()
                .change_context(DownloadError::ChannelError)?;
        }

        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Extract {
    exclude: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Rule {
    action: Action,
    os: Option<Os>,
}

impl Rule {
    #[must_use]
    pub const fn action(&self) -> &Action {
        &self.action
    }

    #[must_use]
    pub const fn os(&self) -> Option<&Os> {
        self.os.as_ref()
    }

    pub fn passes(&self) -> bool {
        match self.action() {
            Action::Allow => {
                let Some(os) = self.os() else {
                    return true;
                };

                let arch_rule = match os.arch().map(String::as_str) {
                    Some("x86") => cfg!(target_arch = "x86"),
                    Some(_) => todo!("Unknown arch"),
                    None => true,
                };

                let os_rule = match os.name().map(String::as_str) {
                    // windows users pls test
                    #[cfg(target_os = "windows")]
                    Some("windows") => {
                        if let Some(ver) = &rule.os.version {
                            if ver != "^10\\." {
                                panic!("unrecognised windows version: {:?}, please report to https://github.com/glowsquid-launcher/copper/issues with the version you are using", ver);
                            }

                            IsWindows10OrGreater().unwrap_or(false)
                        } else {
                            true
                        }
                    }
                    #[cfg(not(target_os = "windows"))]
                    Some("windows") => false,
                    Some("osx") => cfg!(target_os = "macos"),
                    Some("linux") => cfg!(target_os = "linux"),
                    Some(_) => todo!("Unknown os"),
                    None => true,
                };

                arch_rule && os_rule
            }
            Action::Disallow => todo!("No disallow rules for jvm args"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Action {
    Allow,
    Disallow,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Name {
    Osx,
    Linux,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct JavaVersion {
    component: String,
    major_version: u8,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Logging {
    client: LoggingClient,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[allow(clippy::module_name_repetitions)]
pub struct LoggingClient {
    argument: String,
    file: File,
    #[serde(rename = "type")]
    client_type: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct File {
    id: String,
    sha1: String,
    size: i64,
    url: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum Game {
    GameClass(GameClass),
    String(String),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GameClass {
    rules: Vec<GameRule>,
    value: Value,
}

impl GameClass {
    #[must_use]
    pub fn rules(&self) -> &[GameRule] {
        self.rules.as_ref()
    }

    #[must_use]
    pub const fn value(&self) -> &Value {
        &self.value
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GameRule {
    action: Action,
    features: Features,
}

impl GameRule {
    #[must_use]
    pub const fn action(&self) -> &Action {
        &self.action
    }

    #[must_use]
    pub const fn features(&self) -> &Features {
        &self.features
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Features {
    is_demo_user: Option<bool>,
    has_custom_resolution: Option<bool>,
    has_quick_plays_support: Option<bool>,
    is_quick_play_singleplayer: Option<bool>,
    is_quick_play_multiplayer: Option<bool>,
    is_quick_play_realms: Option<bool>,
}

impl Features {
    #[must_use]
    pub const fn demo_user(&self) -> Option<bool> {
        self.is_demo_user
    }

    #[must_use]
    pub const fn custom_resolution(&self) -> Option<bool> {
        self.has_custom_resolution
    }

    #[must_use]
    pub const fn quick_plays_support(&self) -> Option<bool> {
        self.has_quick_plays_support
    }

    #[must_use]
    pub const fn quick_play_singleplayer(&self) -> Option<bool> {
        self.is_quick_play_singleplayer
    }

    #[must_use]
    pub const fn quick_play_multiplayer(&self) -> Option<bool> {
        self.is_quick_play_multiplayer
    }

    #[must_use]
    pub const fn quick_play_realms(&self) -> Option<bool> {
        self.is_quick_play_realms
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum Value {
    String(String),
    StringArray(Vec<String>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Classifiers {
    natives_linux: Option<Artifact>,
    natives_macos: Option<Artifact>,
    natives_windows: Option<Artifact>,
    natives_osx: Option<Artifact>,
}

impl Classifiers {
    #[must_use]
    pub const fn windows(&self) -> Option<&Artifact> {
        self.natives_windows.as_ref()
    }

    #[must_use]
    pub const fn linux(&self) -> Option<&Artifact> {
        self.natives_linux.as_ref()
    }

    #[must_use]
    pub fn macos(&self) -> Option<&Artifact> {
        self.natives_osx.as_ref().or(self.natives_macos.as_ref())
    }

    #[must_use]
    /// Returns the artifact for the current OS.
    pub fn current_os(&self) -> Option<&Artifact> {
        if cfg!(target_os = "windows") {
            self.windows()
        } else if cfg!(target_os = "macos") {
            self.macos()
        } else if cfg!(target_os = "linux") {
            self.linux()
        } else {
            None
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Natives {
    linux: Option<String>,
    osx: Option<String>,
    windows: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Os {
    name: Option<String>,
    version: Option<String>,
    arch: Option<String>,
}

impl Os {
    #[must_use]
    pub const fn name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    #[must_use]
    pub const fn version(&self) -> Option<&String> {
        self.version.as_ref()
    }

    #[must_use]
    pub const fn arch(&self) -> Option<&String> {
        self.arch.as_ref()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Mappings {
    sha1: String,
    size: i64,
    url: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum Jvm {
    String(String),
    Class(JvmClassRule),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JvmClassRule {
    rules: Vec<Rule>,
    value: Value,
}

impl JvmClassRule {
    #[must_use]
    pub const fn value(&self) -> &Value {
        &self.value
    }

    #[must_use]
    pub fn rules(&self) -> &[Rule] {
        self.rules.as_ref()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Arguments {
    #[serde(default)]
    pub(crate) game: Vec<Game>,
    #[serde(default)]
    pub(crate) jvm: Vec<Jvm>,
}

impl Arguments {
    #[must_use]
    pub fn game(&self) -> &[Game] {
        self.game.as_ref()
    }

    #[must_use]
    pub fn jvm(&self) -> &[Jvm] {
        self.jvm.as_ref()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Library {
    downloads: LibraryDownloads,
    name: String,
    rules: Option<Vec<Rule>>,
    natives: Option<Natives>,
    extract: Option<Extract>,
}

impl Library {
    #[must_use]
    pub fn check_rules_passes(&self) -> bool {
        self.rules
            .as_ref()
            .map_or(true, |rules| rules.iter().all(Rule::passes))
    }

    #[must_use]
    pub fn parse_name(&self) -> Option<(&str, &str, &str)> {
        let mut split = self.name.split(':');
        let package = split.next()?;
        let name = split.next()?;
        let version = split.next()?;

        Some((package, name, version))
    }

    #[must_use]
    pub const fn downloads(&self) -> &LibraryDownloads {
        &self.downloads
    }

    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LibraryDownloads {
    artifact: Option<Artifact>,
    classifiers: Option<Classifiers>,
}

impl LibraryDownloads {
    #[must_use]
    pub const fn artifact(&self) -> Option<&Artifact> {
        self.artifact.as_ref()
    }

    #[must_use]
    pub const fn classifiers(&self) -> Option<&Classifiers> {
        self.classifiers.as_ref()
    }
}

#[derive(Debug, Clone)]
pub struct LibraryDownloader {
    libraries: Vec<Library>,
    libraries_directory: PathBuf,

    client: reqwest::Client,
    sender: Option<UnboundedSender<DownloadMessage<Artifact>>>,

    max_concurrent_downloads: usize,
}

impl LibraryDownloader {
    #[must_use]
    pub fn new(
        libraries: Vec<Library>,
        libraries_directory: PathBuf,
        client: reqwest::Client,
        max_concurrent_downloads: usize,
    ) -> Self {
        Self {
            libraries,
            client,
            libraries_directory,
            sender: None,
            max_concurrent_downloads,
        }
    }
}

#[async_trait::async_trait]
impl Downloader for LibraryDownloader {
    type DownloadItem = Artifact;

    fn create_channel(&mut self) -> UnboundedReceiver<DownloadMessage<Self::DownloadItem>> {
        let (sender, receiver) = mpsc::unbounded_channel();
        self.sender = Some(sender);

        receiver
    }

    async fn download(&self, item: Self::DownloadItem) -> error_stack::Result<(), DownloadError> {
        item.download(
            self.libraries_directory.clone(),
            &self.client,
            self.sender.as_ref().ok_or(DownloadError::ChannelError)?,
        )
        .await?;

        self.sender
            .as_ref()
            .ok_or(DownloadError::ChannelError)?
            .send(DownloadMessage::Downloaded(item.clone()))
            .into_report()
            .change_context(DownloadError::ChannelError)
    }

    async fn download_all(self: Arc<Self>) -> error_stack::Result<(), DownloadError> {
        let new_self = self.clone();
        let libraries = new_self
            .libraries
            .iter()
            .cloned()
            .filter(Library::check_rules_passes);

        let tasks = stream::iter(libraries)
            .map(|object| {
                let cloned_self = self.clone();
                tokio::spawn(async move {
                    if let Some(artifact) = object.downloads().artifact() {
                        match cloned_self.download(artifact.clone()).await {
                            Ok(it) => it,
                            Err(err) => return Err(err),
                        };
                    };

                    if let Some(native) = object
                        .downloads()
                        .classifiers()
                        .and_then(Classifiers::current_os)
                    {
                        match cloned_self.download(native.clone()).await {
                            Ok(it) => it,
                            Err(err) => return Err(err),
                        };
                    };

                    Ok(())
                })
            })
            .buffer_unordered(self.max_concurrent_downloads);

        tasks
            .try_collect::<Vec<_>>()
            .await
            .into_report()
            .change_context(DownloadError::JoinError)?
            .into_iter()
            .collect::<Result<_, _>>()?;

        new_self
            .sender
            .clone()
            .ok_or(DownloadError::ChannelError)?
            .send(DownloadMessage::DownloadedAll)
            .into_report()
            .change_context(DownloadError::ChannelError)?;

        Ok(())
    }
}
