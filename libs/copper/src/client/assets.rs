use std::{
    cmp::min,
    collections::HashMap,
    path::{Path, PathBuf},
    sync::Arc,
};

use error_stack::{report, Result, ResultExt};
use futures::{stream, StreamExt, TryStreamExt};
use sha1::{Digest, Sha1};
use tokio::{io::AsyncWriteExt, sync::mpsc};

use crate::{
    api,
    downloader::{DownloadChannelSender, DownloadError, DownloadMessage, Downloader},
};

#[derive(Debug, Clone)]
/// An object in the asset index
pub struct Object {
    /// The hash of the object
    hash: String,
    /// The size of the object in bytes
    size: u64,
}

/// Getter methods
impl Object {
    #[must_use]
    ///T he hash of the object
    pub fn hash(&self) -> &str {
        &self.hash
    }

    #[must_use]
    /// The first two characters of the hash
    pub fn hash_start(&self) -> &str {
        &self.hash[..2]
    }

    #[must_use]
    /// The size of the object in bytes
    pub const fn size(&self) -> u64 {
        self.size
    }

    /// Downloads the object to the specified assets directory,
    /// appending the required path to the assets directory (`objects/{object.hash_start}/{object.hash}`)
    ///
    /// It will redownload the file if it already exists and the hash does not match
    ///
    /// # Errors
    /// Errors if the download fails or if the file cannot be created
    async fn download(
        &self,
        assets_path: &Path,
        client: &reqwest::Client,
        sender: &DownloadChannelSender<Self>,
    ) -> Result<(), DownloadError> {
        let path = assets_path
            .join("objects")
            .join(self.hash_start())
            .join(&self.hash);

        if path.try_exists().change_context(DownloadError::IoError)? {
            let mut file = std::fs::File::open(&path).change_context(DownloadError::IoError)?;
            let mut hasher = Sha1::new();
            std::io::copy(&mut file, &mut hasher).change_context(DownloadError::IoError)?;

            let file_hash = hasher.finalize();

            if self.hash == hex::encode(file_hash) {
                return Ok(());
            }
        }

        let url = format!(
            "https://resources.download.minecraft.net/{}/{}",
            self.hash_start(),
            self.hash
        );

        let mut response = client
            .get(&url)
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

            let new = min(downloaded + (item.len() as u64), self.size());
            downloaded = new;

            sender
                .send(DownloadMessage::DownloadProgress(self.clone(), new))
                .change_context(DownloadError::ChannelError)?;
        }

        Ok(())
    }
}

impl From<api::asset_index::Object> for Object {
    fn from(value: api::asset_index::Object) -> Self {
        Self {
            hash: value.hash,
            size: value.size,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Assets(HashMap<String, Object>);

impl From<api::asset_index::Assets> for Assets {
    fn from(value: api::asset_index::Assets) -> Self {
        Assets(
            value
                .objects
                .into_iter()
                .map(|(k, v)| (k, v.into()))
                .collect(),
        )
    }
}

#[derive(Debug, Clone)]
pub struct AssetDownloader {
    assets: Assets,
    assets_directory: PathBuf,

    client: reqwest::Client,
    sender: Option<DownloadChannelSender<Object>>,

    max_concurrent_downloads: usize,
}

impl AssetDownloader {
    #[must_use]
    pub fn new(
        assets: Assets,
        assets_directory: PathBuf,
        client: reqwest::Client,
        max_concurrent_downloads: usize,
    ) -> Self {
        Self {
            assets,
            assets_directory,
            client,
            sender: None,
            max_concurrent_downloads,
        }
    }
}

impl Downloader for AssetDownloader {
    type DownloadItem = Object;

    fn create_channel(&mut self) -> crate::downloader::DownloadChannelReceiver<Self::DownloadItem> {
        let (sender, receiver) = mpsc::unbounded_channel();
        self.sender = Some(sender);

        receiver
    }

    async fn download_all(self: Arc<Self>) -> Result<(), DownloadError> {
        let new_self = self.clone();

        let tasks = stream::iter(self.assets.0.values().cloned())
            .map(|object| {
                let cloned_self = self.clone();
                tokio::spawn(async move {
                    match object
                        .download(
                            &cloned_self.assets_directory,
                            &cloned_self.client,
                            &cloned_self.sender.as_ref().unwrap(),
                        )
                        .await
                    {
                        Ok(_) => cloned_self
                            .sender
                            .as_ref()
                            .ok_or(DownloadError::ChannelError)?
                            .send(DownloadMessage::Downloaded(object))
                            .change_context(DownloadError::ChannelError),
                        Err(e) => Err(e),
                    }
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
