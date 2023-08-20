use std::{
    collections::HashMap,
    net::SocketAddr,
    path::PathBuf,
    sync::{Arc, OnceLock},
};

use axum::{
    extract::{Query, State},
    routing::get,
    Router,
};
use copper::{
    assets::version,
    auth::{
        structs::{MinecraftToken, OauthCode},
        MSauth,
    },
    downloader::{DownloadMessage, Downloader},
    launcher::{self, AuthenticationDetails, LauncherBuilder, RamSize},
};
use error_stack::Report;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use oauth2::{CsrfToken, PkceCodeVerifier};
use tokio::{io::AsyncBufReadExt, sync::mpsc};
use tracing::{error, info};
use tracing_subscriber::{fmt::format::PrettyFields, prelude::*};

extern crate axum;
extern crate copper;
extern crate error_stack;
extern crate indicatif;
extern crate java_locator;
extern crate tokio;
extern crate tracing;
extern crate tracing_subscriber;

// glowsquids details. If you are going to make your own app, please do not use these.
// these are just for ease of use.
const CLIENT_ID: &str = "2aa32806-92e3-4242-babc-392ac0f0fd30";
const CLIENT_SECRET: &str = "a8F8Q~v58Hfe4nypPkCoiVSCEBG3WKcnYx9uXc-2";

static CSRF_TOKEN: OnceLock<CsrfToken> = OnceLock::new();
static PKCE_VERIFIER: OnceLock<PkceCodeVerifier> = OnceLock::new();
static TOKEN: OnceLock<MinecraftToken> = OnceLock::new();

#[derive(Clone)]
struct AppState {
    shutdown_signal: mpsc::Sender<()>,
    oauth: MSauth,
}

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

    info!("Setting up auth");

    info!("Initializing oauth2 client...");
    // now lets start!
    // we need a redirect uri so that the oauth server can redirect the user back to our server
    let redirect_uri = "http://localhost:3000/code".to_string();
    let oauth = MSauth::new(
        redirect_uri,
        CLIENT_ID.to_string(),
        CLIENT_SECRET.to_string(),
    )
    .expect("To be able to create client");

    info!("Initializing server...");

    // some server setup. You will probably need to set it up using your preferred framework of
    // choice
    let (shutdown, send) = mpsc::channel(1);
    let router = Router::new()
        .route("/code", get(get_code))
        .with_state(AppState {
            shutdown_signal: shutdown,
            oauth: oauth.clone(),
        });

    let listener = SocketAddr::from(([127, 0, 0, 1], 3000));
    let server = tokio::spawn(
        axum::Server::bind(&listener)
            .serve(router.into_make_service())
            .with_graceful_shutdown(shutdown_server(send)),
    );

    info!("Server initialized on port 3000!...");

    // get the auth url and csrf token
    // the csrf token is to prevent Cross Site Request Forgery
    // read more about it at https://en.wikipedia.org/wiki/Cross-site_request_forgery
    // the tl;dr is that it is a security measure to prevent malicious websites from impersonating
    // us
    let (auth_url, csrf_token, pkce) = oauth.get_auth_info();

    // now that we have the verifier and token, we can set it so it can be accessed by the server
    // this is not the best way to do it, but it is the easiest
    PKCE_VERIFIER
        .set(pkce)
        .expect("To be able to set pkce verifier");

    CSRF_TOKEN
        .set(csrf_token)
        .expect("To be able to set csrf token");

    info!("Please go to this url to authenticate: {}", auth_url);

    server.await.unwrap().unwrap();

    info!("Authenticated! Token: {:?}", TOKEN.get());

    info!("Downloading minecraft manifest");
    let manifest = version::Manifest::get().await.unwrap();

    info!("Downloaded minecraft manifest");

    let latest = manifest.latest_release();
    info!("Downloading manifest for latest release: {}", latest.id());

    let manifest = latest.download().await.unwrap();

    let cwd = std::env::current_dir().unwrap();
    let minecraft_dir = cwd.join(".minecraft");

    let manifest_dir = minecraft_dir.join("versions").join(latest.id());

    manifest
        .save_to_disk(&manifest_dir.join(format!("{}.json", latest.id())))
        .await
        .unwrap();

    let token = TOKEN.get().unwrap();
    let profile = oauth.get_minecraft_profile(token).await.unwrap();

    let auth_details = AuthenticationDetails {
        authenticator: oauth,
        auth_details: token.clone(),
        minecraft_profile: profile,
        is_demo_user: false,
    };

    let java_home = PathBuf::from(java_locator::locate_java_home().unwrap());
    let java_path = match std::env::consts::OS {
        "windows" => java_home.join("bin").join("javaw.exe"),
        _ => java_home.join("bin").join("java"),
    };

    let mut launcher = LauncherBuilder::default()
        .authentication_details(auth_details)
        .custom_resolution(None)
        .jar_path(manifest_dir.join(format!("{}.jar", latest.id())))
        .game_directory(minecraft_dir.clone())
        .assets_directory(minecraft_dir.join("assets"))
        .libraries_directory(minecraft_dir.join("libraries"))
        .version_manifest_path(manifest_dir.join(format!("{}.json", latest.id())))
        .is_snapshot(false)
        .version_name("Vanilla")
        .ram_size(RamSize {
            min: "1024Mb".to_string(),
            max: "4096Mb".to_string(),
        })
        .java_path(java_path)
        .launcher_name("copper")
        .launcher_version(env!("CARGO_PKG_VERSION").to_string())
        .quickplay(None)
        .http_client(reqwest::Client::new())
        .manifest(manifest.clone())
        .build()
        .unwrap();

    info!("Launcher created. Downloading files");

    let index = manifest.asset_index().download().await.unwrap();

    let mut downloader =
        launcher::Downloader::new(&launcher, index, manifest.libraries().to_vec(), 16);
    let mut reciever = downloader.create_channel();

    let mut bars = HashMap::new();
    let m = MultiProgress::new();
    let sty = ProgressStyle::with_template(
        "{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})"
    )
    .unwrap()
    .progress_chars("##-");

    let downloader = Arc::new(downloader);
    let task_downloader = downloader.clone();
    let task = tokio::task::spawn(async move {
        task_downloader.download_all().await.unwrap();
    });

    info!("Watching for messages");
    while let Some(dl) = reciever.recv().await {
        match dl {
            DownloadMessage::Downloaded(object) => {
                let hash = match object {
                    launcher::DownloadItem::Asset(asset) => asset.hash().to_owned(),
                    launcher::DownloadItem::Library(library) => library.sha1().to_owned(),
                    launcher::DownloadItem::Client(_) => "client".to_owned(),
                };

                let Some((_, bar)): Option<(String, ProgressBar)> = bars.remove(&hash) else {
                    continue;
                };

                bar.finish();
                m.remove(&bar);
                m.println(format!("Downloaded {}", hash)).unwrap();
            }
            DownloadMessage::DownloadedAll => {
                m.println("Downloaded all objects. Joining tasks").unwrap();
                // we're done, make sure to break
                break;
            }
            DownloadMessage::DownloadProgress(object, how_much) => {
                let hash = match object {
                    launcher::DownloadItem::Asset(ref asset) => asset.hash().to_owned(),
                    launcher::DownloadItem::Library(ref library) => library.sha1().to_owned(),
                    launcher::DownloadItem::Client(_) => "client".to_owned(),
                };

                let size = match object {
                    launcher::DownloadItem::Asset(asset) => asset.size(),
                    launcher::DownloadItem::Library(library) => library.size(),
                    launcher::DownloadItem::Client(client) => client.size(),
                };

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

    info!("Done downloading files. Launching the game");

    let game_manager = launcher.launch().await.unwrap();

    let stdout_task = tokio::spawn(async move {
        let mut lines = game_manager.stdout.lines();
        while let Some(buf) = lines.next_line().await.unwrap() {
            info!("STDOUT: {}", buf);
        }
    });

    let stderr_task = tokio::spawn(async move {
        let mut lines = game_manager.stderr.lines();
        while let Some(line) = lines.next_line().await.unwrap() {
            error!("STDERR: {}", line);
        }
    });

    info!("Game launched. Waiting for it to exit");

    stdout_task.await.unwrap();
    stderr_task.await.unwrap();
    game_manager.exit_handle.await.unwrap();
}

async fn shutdown_server(mut signal_to_shutdown: mpsc::Receiver<()>) {
    signal_to_shutdown.recv().await.unwrap();
    info!("Shutting down auth server...");
}

async fn get_code(Query(code): Query<OauthCode>, State(state): State<AppState>) {
    info!("Received code: {:?}. Authenticating...", code);

    let csrf = CSRF_TOKEN
        .get()
        .expect("To be able to get CSRF token")
        .clone();

    let pkce = PKCE_VERIFIER
        .get()
        .expect("To be able to get pkce verifier");

    let ms_token = state
        .oauth
        .get_ms_access_token(code, csrf, pkce)
        .await
        .expect("To be able to get token");

    let token = state
        .oauth
        .get_minecraft_token(ms_token.clone())
        .await
        .expect("To be able to get token");

    TOKEN.set(token).expect("To be able to set token");
    state.shutdown_signal.send(()).await.unwrap();
}
