use error_stack::Result;
use std::{
    error::Error,
    fmt::{self, Display, Formatter},
    sync::Arc,
};

use tokio::sync::mpsc::UnboundedReceiver;

#[derive(Debug)]
pub enum DownloadError {
    ReqwestError,
    IoError,
    JoinError,
    ChannelError,
}

impl Display for DownloadError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::ReqwestError => "reqwest error",
            Self::IoError => "io error",
            Self::JoinError => "join error",
            Self::ChannelError => "channel error",
        })
    }
}

impl Error for DownloadError {}

#[derive(Debug)]
pub enum DownloadMessage<T> {
    /// A file was successfully downloaded
    Downloaded(T),
    /// When you recieve this event, you can be sure that all file have been downloaded.
    /// This is only sent once. You must close the channel after this, else your program will hang
    DownloadedAll,
    /// (T, downloaded bytes)
    DownloadProgress(T, u64),
}

#[async_trait::async_trait]
pub trait Downloader {
    type DownloadItem;

    /// Creates a channel for downloading
    fn create_channel(&mut self) -> UnboundedReceiver<DownloadMessage<Self::DownloadItem>>;

    /// Downloads an object
    ///
    /// See the downloaders documentation for more information. Usually appends the path to the
    /// main directory.
    async fn download(&self, item: Self::DownloadItem) -> Result<(), DownloadError>;

    /// Downloads all files for this specific downloader. Requires an Arc for multithreading
    /// purposes
    ///
    /// See the downloaders documentation for more information. Usually appends the path to the
    /// main directory.
    async fn download_all(self: Arc<Self>) -> Result<(), DownloadError>;
}
