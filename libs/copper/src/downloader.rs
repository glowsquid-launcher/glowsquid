use error_stack::Result;
use std::{
    error::Error,
    fmt::{self, Display, Formatter},
    future::Future,
    sync::Arc,
};

use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

#[derive(Debug)]
/// An error that can occur when downloading a file
pub enum DownloadError {
    /// An error occured when requesting/downloading the file
    ReqwestError,
    /// An I/O error occured when writing or reading the file
    IoError,
    /// An error occured when joining the download task
    JoinError,
    /// An error occured during channel communication (such as sending progress updates)
    ChannelError,
    /// An error occured when extracting the downloaded file (if applicable)
    ExtractionError,
}

impl Display for DownloadError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::ReqwestError => "Reqwest Error",
            Self::IoError => "I/O Error",
            Self::JoinError => "Join Error",
            Self::ChannelError => "Channel Error",
            Self::ExtractionError => "Extraction Error",
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

pub type DownloadChannelSender<T> = UnboundedSender<DownloadMessage<T>>;
pub type DownloadChannelReceiver<T> = UnboundedReceiver<DownloadMessage<T>>;

pub trait Downloader {
    type DownloadItem;

    /// Creates a channel for downloading
    fn create_channel(&mut self) -> DownloadChannelReceiver<Self::DownloadItem>;

    /// Downloads all files for this specific downloader. Requires an Arc for multithreading
    /// purposes
    ///
    /// See the downloaders documentation for more information. Usually appends the path to the
    /// main directory.
    fn download_all(self: Arc<Self>) -> impl Future<Output = Result<(), DownloadError>> + Send;
}
