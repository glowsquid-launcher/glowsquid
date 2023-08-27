use error_stack::{report, Result, ResultExt};
use futures::{stream, StreamExt, TryStreamExt};
use serde::{Deserialize, Serialize};
use std::{cmp::min, collections::HashMap, path::PathBuf, sync::Arc};
use tokio::{
    io::AsyncWriteExt,
    sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender},
};

use crate::downloader::{DownloadError, DownloadMessage, Downloader};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Assets {
    objects: HashMap<String, Object>,
}

#[derive(Debug, Clone)]
pub struct AssetDownloader {
    assets: Assets,
    assets_directory: PathBuf,

    client: reqwest::Client,
    sender: Option<UnboundedSender<DownloadMessage<Object>>>,

    max_concurrent_downloads: usize,
}

impl AssetDownloader {
    #[must_use]
    pub const fn new(
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

#[async_trait::async_trait]
impl Downloader for AssetDownloader {
    type DownloadItem = Object;

    fn create_channel(&mut self) -> UnboundedReceiver<DownloadMessage<Self::DownloadItem>> {
        let (sender, receiver) = unbounded_channel();
        self.sender = Some(sender);
        receiver
    }

    async fn download(&self, item: Self::DownloadItem) -> Result<(), DownloadError> {
        let path = self
            .assets_directory
            .join(item.hash_start())
            .join(&item.hash);

        item.download(
            path,
            &self.client,
            self.sender.as_ref().ok_or(DownloadError::ChannelError)?,
        )
        .await?;

        self.sender
            .as_ref()
            .ok_or(DownloadError::ChannelError)?
            .send(DownloadMessage::Downloaded(item.clone()))
            .change_context(DownloadError::ChannelError)
    }

    async fn download_all(self: Arc<Self>) -> Result<(), DownloadError> {
        let new_self = self.clone();
        let tasks = stream::iter(self.assets.objects.values().cloned())
            .map(|object| {
                let cloned_self = self.clone();
                tokio::spawn(async move { cloned_self.download(object).await })
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

/// Getter methods
impl Assets {
    #[must_use]
    pub const fn objects(&self) -> &HashMap<String, Object> {
        &self.objects
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Object {
    hash: String,
    size: u64,
}

/// High-level API
impl Object {
    /// Downloads the object to the specified path using the specified client
    ///
    /// No appending is done to the path, so you must specify the full path
    /// If you want to download to the assets directory with the appended path, use
    /// [`Assets::download_object`]
    ///
    /// It will NOT redownload the file if it already exists
    ///
    /// # Errors
    /// Errors if the download fails or if the file cannot be created
    pub async fn download(
        &self,
        path: PathBuf,
        client: &reqwest::Client,
        sender: &UnboundedSender<DownloadMessage<Self>>,
    ) -> Result<(), DownloadError> {
        if path.try_exists().change_context(DownloadError::IoError)? {
            return Ok(());
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

/// Getter methods
impl Object {
    #[must_use]
    pub fn hash(&self) -> &str {
        &self.hash
    }

    #[must_use]
    pub fn hash_start(&self) -> &str {
        &self.hash[..2]
    }

    #[must_use]
    pub const fn size(&self) -> u64 {
        self.size
    }
}
