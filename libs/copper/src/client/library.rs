use std::{
    error::Error,
    fmt,
    path::{Path, PathBuf},
    sync::Arc,
};

use error_stack::{report, Report, ResultExt};
use futures::{stream, StreamExt, TryStreamExt};
use tokio::{fs, sync::mpsc};

use crate::{
    api::{self, client::Artifact},
    downloader::{
        DownloadChannelReceiver, DownloadChannelSender, DownloadError, DownloadMessage, Downloader,
    },
};

#[derive(Debug, Clone)]
pub struct Library {
    /// The package/namespace of the library
    package: String,
    /// The name of the library
    name: String,
    /// The version of the library
    version: String,
    /// The downloads for the library
    downloads: api::client::LibraryDownloads,
    /// The rules for the library
    ///
    /// If the rules are not met, the library will not be processed
    rules: Vec<api::client::Rule>,
    /// The natives for the library
    ///
    /// Depending on the operating system, the natives will be extracted
    natives: api::client::Natives,
    /// The extract information for the library
    /// If this is `None`, the library is not extracted
    extract: Option<api::client::Extract>,
}

impl TryFrom<api::client::Library> for Library {
    type Error = Report<LauncherLibraryError>;

    fn try_from(value: api::client::Library) -> Result<Self, Self::Error> {
        let (package, name, version) = value
            .parse_name()
            .ok_or(report!(LauncherLibraryError::InvalidName))?;

        Ok(Self {
            package: package.to_owned(),
            name: name.to_owned(),
            version: version.to_owned(),
            downloads: value.downloads,
            extract: value.extract,
            rules: value.rules.unwrap_or_default(),
            natives: value.natives.unwrap_or_default(),
        })
    }
}

#[derive(Debug)]
pub enum LauncherLibraryError {
    InvalidName,
}

impl fmt::Display for LauncherLibraryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::InvalidName => {
                "Invalid package name. It must be in the format: package:name:version"
            }
        })
    }
}

impl Error for LauncherLibraryError {}

impl Library {
    pub fn canonical_name(&self) -> String {
        format!("{}:{}:{}", self.package, self.name, self.version)
    }

    #[must_use]
    pub fn check_rules_passes(&self) -> bool {
        self.rules.iter().all(api::client::Rule::passes)
    }

    pub(crate) fn downloads(&self) -> &api::client::LibraryDownloads {
        &self.downloads
    }

    /// Downloads the library artifacts and classifiers to the specified base directory,
    /// appending the required path to the base directory (`{artifact.path}/{artifact.name}`)
    pub async fn download(
        &self,
        base_path: &Path,
        client: &reqwest::Client,
        sender: &DownloadChannelSender<Artifact>,
    ) -> error_stack::Result<(), DownloadError> {
        if let Some(artifact) = &self.downloads.artifact {
            artifact.download(base_path, client, sender).await?;
        };

        if let Some(native) = self.native_for_current_os() {
            native.download(base_path, client, sender).await?;
            self.extract_native(native, base_path).await?;
        }
        Ok(())
    }

    #[must_use]
    /// Returns the native artifact for the current OS.
    pub fn native_for_current_os(&self) -> Option<&Artifact> {
        let classifiers = self.downloads.classifiers.as_ref()?;

        if cfg!(target_os = "windows") && self.natives.windows.is_some() {
            classifiers.windows()
        } else if cfg!(target_os = "macos") {
            classifiers.macos()
        } else if cfg!(target_os = "linux") && self.natives.linux.is_some() {
            classifiers.linux()
        } else {
            None
        }
    }

    async fn extract_native(
        &self,
        native: &Artifact,
        base_path: &Path,
    ) -> error_stack::Result<(), DownloadError> {
        if self
            .extract
            .as_ref()
            .is_some_and(|ex| !ex.exclude.is_empty())
        {
            todo!("Handle extraction exclude rules");
        }

        let path = base_path.join(&native.path());

        if !path.try_exists().change_context(DownloadError::IoError)? {
            return Err(report!(DownloadError::ExtractionError)
                .attach_printable(format!("Native does not exist at path: {}", path.display())));
        }

        let file = std::fs::File::open(&path).change_context(DownloadError::IoError)?;

        let mut archive =
            zip::ZipArchive::new(file).change_context(DownloadError::ExtractionError)?;

        // format is {libraries_dir}/{os}/{arch}
        let os = std::env::consts::OS;
        let arch = std::env::consts::ARCH.replace("x86_64", "x64");
        let native_extract_dir = base_path.join(os).join(arch);

        fs::create_dir_all(&native_extract_dir)
            .await
            .change_context(DownloadError::IoError)?;

        archive
            .extract(&native_extract_dir)
            .change_context(DownloadError::ExtractionError)?;

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct LibraryDownloader {
    libraries: Vec<Library>,
    libraries_directory: PathBuf,

    client: reqwest::Client,
    sender: Option<DownloadChannelSender<Artifact>>,

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

impl Downloader for LibraryDownloader {
    type DownloadItem = Artifact;

    fn create_channel(&mut self) -> DownloadChannelReceiver<Self::DownloadItem> {
        let (sender, receiver) = mpsc::unbounded_channel();
        self.sender = Some(sender);

        receiver
    }

    async fn download_all(self: Arc<Self>) -> error_stack::Result<(), DownloadError> {
        let new_self = self.clone();

        // bug: this doesn't work if you filter first
        #[allow(clippy::iter_overeager_cloned)]
        let libraries = new_self
            .libraries
            .iter()
            .cloned()
            .filter(Library::check_rules_passes);

        let tasks = stream::iter(libraries)
            .map(|library| {
                let cloned_self = self.clone();
                tokio::spawn(async move {
                    match library
                        .download(
                            &cloned_self.libraries_directory,
                            &cloned_self.client,
                            cloned_self.sender.as_ref().unwrap(),
                        )
                        .await
                    {
                        Ok(()) => (),
                        Err(e) => return Err(e),
                    };

                    Ok(())
                })
            })
            .buffer_unordered(self.max_concurrent_downloads);

        tasks
            .try_collect::<Vec<_>>()
            .await
            .change_context(DownloadError::JoinError)?
            .into_iter()
            .collect::<Result<_, _>>()?;

        new_self
            .sender
            .clone()
            .ok_or(DownloadError::ChannelError)?
            .send(DownloadMessage::DownloadedAll)
            .change_context(DownloadError::ChannelError)?;

        Ok(())
    }
}
