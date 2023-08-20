use std::{collections::HashMap, sync::Arc};

use copper::{
    assets::{asset_index::AssetDownloader, version},
    downloader::{DownloadMessage, Downloader},
};
use error_stack::Report;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use tracing::info;
use tracing_subscriber::{fmt::format::PrettyFields, prelude::*};

extern crate copper;
extern crate error_stack;
extern crate indicatif;
extern crate tokio;
extern crate tracing;
extern crate tracing_subscriber;

#[tokio::main]
async fn main() {
    // some setup for logging
    Report::set_color_mode(error_stack::fmt::ColorMode::Color);

    let error_handler = tracing_error::ErrorLayer::new(PrettyFields::new());

    tracing_subscriber::fmt()
        .pretty()
        .with_env_filter(
            tracing_subscriber::EnvFilter::builder()
                .with_default_directive(tracing::Level::INFO.into())
                .from_env_lossy(),
        )
        .finish()
        .with(error_handler)
        .init();

    info!("Downloading minecraft manifest");
    let manifest = version::Manifest::get().await.unwrap();

    info!("Downloaded minecraft manifest");

    let latest = manifest.latest_release();
    info!("Downloading manifest for latest release: {}", latest.id());

    let manifest = latest.download().await.unwrap();
    info!("Downloaded manifest for latest release: {}", latest.id());
    info!("Downloading asset index");

    let cwd = std::env::current_dir().unwrap();
    let assets = cwd.join(".minecraft").join("assets");
    info!("Assets directory: {}", assets.display());

    let index = manifest.asset_index().download().await.unwrap();

    let mut download_index = AssetDownloader::new(index, assets, reqwest::Client::new(), 16);
    let mut reciever = download_index.create_channel();

    info!("Downloaded asset index");
    info!("Downloading assets");

    let downloader = Arc::new(download_index);

    let mut bars = HashMap::new();
    let m = MultiProgress::new();
    let sty = ProgressStyle::with_template(
        "{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})"
    )
    .unwrap()
    .progress_chars("##-");

    let task_downloader = downloader.clone();
    let task = tokio::task::spawn(async move {
        task_downloader.download_all().await.unwrap();
    });

    info!("Watching for messages");
    while let Some(dl) = reciever.recv().await {
        match dl {
            DownloadMessage::Downloaded(object) => {
                let hash = object.hash().to_owned();
                let Some((_, bar)): Option<(String, ProgressBar)> = bars.remove(&hash) else {
                    continue;
                };

                bar.finish();
                m.remove(&bar);
            }
            DownloadMessage::DownloadedAll => {
                m.println("Downloaded all objects. Joining tasks").unwrap();
                // we're done, make sure to break
                break;
            }
            DownloadMessage::DownloadProgress(object, how_much) => {
                let hash = object.hash().to_owned();
                let size = object.size();
                let bar = bars.entry(hash.clone()).or_insert_with(|| {
                    let bar = m.add(indicatif::ProgressBar::new(size));
                    bar.set_style(sty.clone());
                    bar.set_message(hash.clone());

                    (hash.clone(), bar)
                });

                bar.1.set_position(how_much);
            }
        }
    }

    task.await.unwrap();

    info!("Done");
}
