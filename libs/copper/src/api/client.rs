use std::{
    cmp::min,
    error::Error,
    fmt::Display,
    path::{Path, PathBuf},
    sync::Arc,
};

use error_stack::{report, Result as ErrorStackResult, ResultExt};
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use serde_json::to_string;
use sha1::{Digest, Sha1};
use tokio::{
    fs,
    io::AsyncWriteExt,
    sync::mpsc::{self, UnboundedReceiver, UnboundedSender},
};
use tracing::debug;

#[cfg(target_os = "windows")]
use winsafe::IsWindows10OrGreater;

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
        let value = to_string(self).change_context(SaveError::SerializeError)?;

        let directory = path.parent().ok_or_else(|| report!(SaveError::IOError))?;

        if !directory.exists() {
            debug!("Creating directory {}", directory.display());
            fs::create_dir_all(directory)
                .await
                .change_context(SaveError::IOError)?;
        }

        debug!("Writing manifest to {}", path.display());
        fs::write(path, value)
            .await
            .change_context(SaveError::IOError)
    }
}

// Thank you QuickType for generating these :)

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Manifest {
    pub(crate) asset_index: AssetIndex,
    pub(crate) assets: String,
    pub(crate) downloads: Downloads,
    pub(crate) id: String,
    pub(crate) libraries: Vec<Library>,
    pub(crate) main_class: String,
    /// Used before 1.13
    pub(crate) minecraft_arguments: Option<String>,
    pub(crate) arguments: Option<Arguments>,
    pub(crate) minimum_launcher_version: i64,
    pub(crate) java_version: Option<JavaVersion>,
    pub(crate) release_time: String,
    pub(crate) time: String,
    #[serde(rename = "type")]
    pub(crate) manifest_type: Type,
    pub(crate) logging: Option<Logging>,
    pub(crate) compliance_level: Option<u8>,
    /// What to merge from. During a merge, it will take the current manifest and merge this manifest into it
    pub(crate) inherits_from: Option<PathBuf>,
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

    pub async fn download(&self, item: DownloadClass) -> ErrorStackResult<(), DownloadError> {
        if self
            .path
            .try_exists()
            .change_context(DownloadError::IoError)?
        {
            return Ok(());
        }

        let mut response = self
            .client
            .get(&item.url)
            .send()
            .await
            .change_context(DownloadError::ReqwestError)?
            .bytes_stream();

        let parent_dir = self
            .path
            .parent()
            .ok_or_else(|| report!(DownloadError::IoError))?;

        tokio::fs::create_dir_all(parent_dir)
            .await
            .change_context(DownloadError::IoError)?;

        let mut file = tokio::fs::File::create(&self.path)
            .await
            .change_context(DownloadError::IoError)?;

        let mut downloaded: u64 = 0;

        while let Some(item) = response.next().await {
            let item = item.change_context(DownloadError::ReqwestError)?;

            file.write_all(&item)
                .await
                .change_context(DownloadError::IoError)?;

            let new = min(downloaded + (item.len() as u64), self.class.size);
            downloaded = new;

            self.sender
                .as_ref()
                .ok_or(DownloadError::ChannelError)?
                .send(DownloadMessage::DownloadProgress(self.class.clone(), new))
                .change_context(DownloadError::ChannelError)?;
        }

        self.sender
            .as_ref()
            .ok_or(DownloadError::ChannelError)?
            .send(DownloadMessage::Downloaded(self.class.clone()))
            .change_context(DownloadError::ChannelError)
    }
}

impl Downloader for ClassDownloader {
    type DownloadItem = DownloadClass;

    fn create_channel(&mut self) -> UnboundedReceiver<DownloadMessage<Self::DownloadItem>> {
        let (sender, receiver) = mpsc::unbounded_channel();
        self.sender = Some(sender);
        receiver
    }

    async fn download_all(self: Arc<Self>) -> ErrorStackResult<(), DownloadError> {
        self.download(self.class.clone()).await?;

        self.sender
            .as_ref()
            .ok_or(DownloadError::ChannelError)?
            .send(DownloadMessage::DownloadedAll)
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
    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn sha1(&self) -> &str {
        &self.sha1
    }

    pub fn size(&self) -> u64 {
        self.size
    }

    /// Downloads the artifact to the given path, appending the path to the given path.
    ///
    /// If there is no artifact or it already exists with the correct hash, it won't be redownloaded
    ///
    /// # Errors
    /// This function will return an error if the artifact is not downloaded successfully.
    pub async fn download(
        &self,
        library_path: &Path,
        client: &reqwest::Client,
        sender: &UnboundedSender<DownloadMessage<Self>>,
    ) -> ErrorStackResult<(), DownloadError> {
        let path = library_path.join(&self.path);

        if path.try_exists().change_context(DownloadError::IoError)? {
            let mut file = std::fs::File::open(&path).change_context(DownloadError::IoError)?;
            let mut hasher = Sha1::new();
            std::io::copy(&mut file, &mut hasher).change_context(DownloadError::IoError)?;

            let file_hash = hasher.finalize();

            if self.sha1 == hex::encode(file_hash) {
                return Ok(());
            }
        }

        let mut response = client
            .get(&self.url)
            .send()
            .await
            .change_context(DownloadError::ReqwestError)?
            .bytes_stream();

        let parent_dir = path
            .parent()
            .ok_or_else(|| report!(DownloadError::IoError))?;

        tokio::fs::create_dir_all(parent_dir)
            .await
            .change_context(DownloadError::IoError)?;

        let mut file = tokio::fs::File::create(path)
            .await
            .change_context(DownloadError::IoError)?;

        let mut downloaded: u64 = 0;

        while let Some(item) = response.next().await {
            let item = item.change_context(DownloadError::ReqwestError)?;

            file.write_all(&item)
                .await
                .change_context(DownloadError::IoError)?;

            let new = min(downloaded + (item.len() as u64), self.size);
            downloaded = new;

            sender
                .send(DownloadMessage::DownloadProgress(self.clone(), new))
                .change_context(DownloadError::ChannelError)?;
        }

        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Extract {
    pub(crate) exclude: Vec<String>,
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

    /// Returns whether the rule passes for the current OS.
    ///
    /// # Panics
    /// Panics if the OS is not recognised. Please report this
    pub fn passes(&self) -> bool {
        match self.action() {
            Action::Allow => {
                let Some(os) = self.os() else {
                    return true;
                };

                let arch_rule = match os.arch().map(String::as_str) {
                    Some("x86") => cfg!(target_arch = "x86"),
                    Some(arch) => todo!("Unknown arch: {arch}. Please report this error to https://github.com/glowsquid-launcher/glowsquid"),
                    None => true,
                };

                let os_rule = match os.name().map(String::as_str) {
                    // windows users pls test
                    #[cfg(target_os = "windows")]
                    Some("windows") => {
                        assert!(!os.version.as_ref().is_some_and(|ver| ver != "^10\\."), "unrecognised windows version: {:?}, please report to https://github.com/glowsquid-launcher/glowsquid/issues with the version you are using", os.version);

                        if os.version.as_ref().is_some_and(|ver| ver == "^10\\.") {
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
    pub(crate) component: String,
    pub(crate) major_version: u8,
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
pub enum GameArgument {
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
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Natives {
    pub(crate) linux: Option<String>,
    pub(crate) osx: Option<String>,
    pub(crate) windows: Option<String>,
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
pub enum JvmArgument {
    String(String),
    Class(JvmClassRule),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JvmClassRule {
    pub(crate) rules: Vec<Rule>,
    pub(crate) value: Value,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Arguments {
    #[serde(default)]
    pub(crate) game: Vec<GameArgument>,
    #[serde(default)]
    pub(crate) jvm: Vec<JvmArgument>,
}

impl Arguments {
    #[must_use]
    pub fn game(&self) -> &[GameArgument] {
        self.game.as_ref()
    }

    #[must_use]
    pub fn jvm(&self) -> &[JvmArgument] {
        self.jvm.as_ref()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct Library {
    pub downloads: LibraryDownloads,
    pub name: String,
    pub rules: Option<Vec<Rule>>,
    pub natives: Option<Natives>,
    pub extract: Option<Extract>,
}

impl Library {
    #[must_use]
    pub fn parse_name(&self) -> Option<(&str, &str, &str)> {
        let mut split = self.name.split(':');
        let package = split.next()?;
        let name = split.next()?;
        let version = split.next()?;

        Some((package, name, version))
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct LibraryDownloads {
    pub artifact: Option<Artifact>,
    pub classifiers: Option<Classifiers>,
}
