use std::{
    collections::HashMap,
    future::IntoFuture,
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
    api::version,
    client::{
        self,
        auth::{AuthenticationInfo, MicrosoftAuthenticator, MinecraftToken, OauthCode},
    },
    downloader::{DownloadMessage, Downloader},
    launcher::{self, AuthenticationDetails, LauncherBuilder, RamSize},
};
use error_stack::Report;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use oauth2::{CsrfToken, PkceCodeVerifier};
use tokio::{io::AsyncBufReadExt, net::TcpListener, sync::mpsc, task};
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
const CLIENT_SECRET: &str = "nky8Q~8lORwTC0OjdxVsgZSs0hCdTcEdec3hNbaP";

static CSRF_TOKEN: OnceLock<CsrfToken> = OnceLock::new();
static PKCE_VERIFIER: OnceLock<PkceCodeVerifier> = OnceLock::new();
static TOKEN: OnceLock<MinecraftToken> = OnceLock::new();

#[derive(Clone)]
struct AppState {
    oauth: MicrosoftAuthenticator,
    shutdown_send: Arc<mpsc::Sender<()>>,
}

#[tokio::main]
async fn main() {
    // some setup for logging
    Report::set_color_mode(error_stack::fmt::ColorMode::Color);

    let error_handler = tracing_error::ErrorLayer::new(PrettyFields::new());

    let (shutdown_sender, mut shutdown_recv) = mpsc::channel(1);
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

    let client = reqwest::Client::new();

    info!("Setting up auth");

    let cwd = std::env::current_dir().unwrap();
    let minecraft_dir = cwd.join(".minecraft");
    let token_path = minecraft_dir.join("token.json");

    // now lets start auth!
    // we need a redirect uri so that the oauth server can redirect the user back to our server
    info!("Initializing oauth2 client...");
    let redirect_uri = "http://localhost:3000/code".to_string();
    let (oauth, auth_info) = MicrosoftAuthenticator::new(
        client.clone(),
        redirect_uri,
        CLIENT_ID.to_string(),
        CLIENT_SECRET.to_string(),
    )
    .expect("To be able to create client");

    // Check if the token.json file already exists. If it does, we can just load the token from disk
    if token_path.exists() {
        info!("Token exists, loading from disk");
        let token = std::fs::read_to_string(token_path).unwrap();
        let mut token: MinecraftToken = serde_json::from_str(&token).unwrap();
        token.refresh(&oauth).await.unwrap();
        TOKEN.set(token).expect("To be able to set token");
    } else {
        info!("Initializing server...");

        // Start the server and wait for the user to authenticate
        let router = Router::new()
            .route("/code", get(get_code))
            .with_state(AppState {
                oauth: oauth.clone(),
                shutdown_send: Arc::new(shutdown_sender),
            });

        let socket_addr = SocketAddr::from(([127, 0, 0, 1], 3000));
        let listener = TcpListener::bind(&socket_addr).await.unwrap();
        let server = task::spawn(
            axum::serve(listener, router.into_make_service())
                .with_graceful_shutdown(async move { shutdown_recv.recv().await.unwrap() })
                .into_future(),
        );

        info!("Server initialized on port 3000!...");

        // get the auth url and csrf token
        // the csrf token is to prevent Cross Site Request Forgery
        // read more about it at https://en.wikipedia.org/wiki/Cross-site_request_forgery
        // the tl;dr is that it is a security measure to prevent malicious websites from impersonating
        // us
        let AuthenticationInfo {
            url,
            csrf_token,
            pkce_verifier,
        } = auth_info;

        // now that we have the verifier and token, we can set it so it can be accessed by the server
        // this is not the best way to do it, but it is the easiest and works for this example

        PKCE_VERIFIER
            .set(pkce_verifier)
            .expect("To be able to set pkce verifier");

        CSRF_TOKEN
            .set(csrf_token)
            .expect("To be able to set csrf token");

        info!("Please go to this url to authenticate: {}", url);

        tokio::spawn(server);

        // cursed way to wait for the token to be set
        while TOKEN.get().is_none() {
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        }

        info!("Saving token to disk. Please delete this once you're done testing");

        // Please do not do this in production. This is just for ease of use
        // In production, you should store the token in a secure way so that it is not exposed in plaintext
        let as_json = serde_json::to_string_pretty(&TOKEN.get().unwrap()).unwrap();

        std::fs::write(token_path, as_json).unwrap();
    }

    info!("Authenticated! Token: {:?}", TOKEN.get());

    // We need to download some metadata about the latest game version before we can launch the game
    info!("Downloading minecraft manifest");
    let manifest = version::Manifest::get().await.unwrap();

    info!("Downloaded minecraft manifest");

    let latest = manifest.latest_release();
    info!("Downloading manifest for latest release: {}", latest.id());

    // This is the actual version manifest file that contains all the information about the version
    // Such as the libraries, assets, java arguments, etc
    let manifest = latest.download().await.unwrap();

    let manifest_dir = minecraft_dir.join("versions").join(latest.id());

    manifest
        .save_to_disk(&manifest_dir.join(format!("{}.json", latest.id())))
        .await
        .unwrap();

    let token = TOKEN.get().unwrap();

    let auth_details = AuthenticationDetails::new(oauth, token.clone(), false);

    // We need to locate the java executable so that we can launch the game
    // In a real application, you would probably want to make sure that the java install is valid for the version
    // and allow the user to specify their own java path if they want to
    let java_home = PathBuf::from(java_locator::locate_java_home().expect("Java not found"));
    let java_path = match std::env::consts::OS {
        // On windows, we need to use javaw.exe to prevent a console window from opening
        "windows" => java_home.join("bin").join("javaw.exe"),
        _ => java_home.join("bin").join("java"),
    };

    // We need to convert the manifest into a format that the launcher can understand
    // This removes a lot of the unnecessary information and makes it easier to work with
    let launcher_manifest: client::manifest::Manifest = manifest.try_into().unwrap();

    // now we can create the launcher and prepare to launch the game
    let mut launcher = LauncherBuilder::default()
        .authentication_details(auth_details)
        .custom_resolution(None)
        .jar_path(manifest_dir.join(format!("{}.jar", latest.id())))
        .game_directory(minecraft_dir.clone())
        .assets_directory(minecraft_dir.join("assets"))
        .libraries_directory(minecraft_dir.join("libraries"))
        .version_manifest_path(manifest_dir.join(format!("{}.json", latest.id())))
        // You should probably get this from the original version manifest, but for now we'll just hardcode it since we're only using the latest stable version
        .is_snapshot(false)
        .version_name(latest.id())
        .ram_size(RamSize {
            min: "1024Mb".to_string(),
            max: "4096Mb".to_string(),
        })
        .java_path(java_path)
        // these should reflect the name and version of your launcher
        .launcher_name("copper")
        .launcher_version(env!("CARGO_PKG_VERSION").to_string())
        // This is for quickplay, which is a feature that allows you to quickly launch the game without any additional setup
        //
        // You could use this to instantly open up a world or server. Pretty handy if you want to have a more intergrated launcher experience
        .quickplay(None)
        // This is a reqwest client that is used to download additional files the game needs
        .http_client(client.clone())
        .manifest(launcher_manifest.clone())
        .build()
        .unwrap();

    // Once the launcher is made, we can add additional arguments to the JVM
    // Here are some examples for debugging and custom wayland support

    // launcher.add_jvm_argument(copper::assets::client::Jvm::String(
    //   "-Dorg.lwjgl.util.Debug=true".to_string()
    // ));

    // launcher.add_jvm_argument(copper::assets::client::Jvm::String(
    //     "-Dorg.lwjgl.glfw.libname=glfw_wayland".to_string()
    // ));

    info!("Launcher created. Downloading files");

    // the asset index is a list of all the assets the game needs to run
    let index = launcher_manifest.asset_index().download().await.unwrap();

    // we also need to save the index, since the game uses it to locate the assets on disk
    let index_dir = minecraft_dir.join("assets").join("indexes");
    index
        .save_to_disk(&index_dir.join(format!("{}.json", launcher_manifest.assets())))
        .await
        .unwrap();

    // We can now start downloading everything the game needs
    let mut downloader = launcher::Downloader::new(&launcher, index.into(), 16);

    // We need to create a channel to watch for download progress
    let mut reciever = downloader.create_channel();

    // Just some download tracking. You can do this however you want
    let mut bars = HashMap::new();
    let m = MultiProgress::new();
    let sty = ProgressStyle::with_template(
        "{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})"
    )
    .unwrap()
    .progress_chars("##-");

    let downloader = Arc::new(downloader);

    // offload the download to a separate task to prevent blocking the main thread, and therefore the progress bars
    let task_downloader = downloader.clone();
    let task = tokio::task::spawn(async move {
        task_downloader.download_all().await.unwrap();
    });

    info!("Watching for messages");
    // every time we get a message, update the progress trackers
    while let Some(dl) = reciever.recv().await {
        match dl {
            DownloadMessage::Downloaded(object) => {
                let download_name = match object {
                    launcher::DownloadItem::Asset(asset) => asset.hash().to_owned(),
                    launcher::DownloadItem::Library(artifact) => artifact.sha1().to_owned(),
                    launcher::DownloadItem::Client(_) => "client".to_owned(),
                };

                let Some((_, bar)): Option<(String, ProgressBar)> = bars.remove(&download_name)
                else {
                    continue;
                };

                bar.finish();
                m.remove(&bar);
                m.println(format!("Downloaded {}", download_name)).unwrap();
            }
            DownloadMessage::DownloadedAll => {
                // This is a final message that tells us that all the files have been downloaded.
                // Useful for when you want to do something after all the files have been downloaded
                m.println("Downloaded all files required. Joining tasks")
                    .unwrap();

                // in our case, we're done, so make sure to break out of the loop
                break;
            }
            DownloadMessage::DownloadProgress(object, how_much) => {
                let download_name = match object {
                    launcher::DownloadItem::Asset(ref asset) => asset.hash().to_owned(),
                    launcher::DownloadItem::Library(ref artifact) => artifact.sha1().to_owned(),
                    launcher::DownloadItem::Client(_) => "client".to_owned(),
                };

                let size = match object {
                    launcher::DownloadItem::Asset(asset) => asset.size(),
                    launcher::DownloadItem::Library(artifact) => artifact.size(),
                    launcher::DownloadItem::Client(client) => client.size(),
                };

                let bar = bars.entry(download_name.clone()).or_insert_with(|| {
                    let bar = m.add(indicatif::ProgressBar::new(size));
                    bar.set_style(sty.clone());
                    bar.set_message(download_name.clone());

                    (download_name.clone(), bar)
                });

                bar.1.set_position(how_much);
            }
        }
    }

    task.await.unwrap();

    info!("Done downloading files. Launching the game");

    // now we can actually launch the game!

    let game_manager = launcher.launch().await.unwrap();

    // These tasks forward the stdout and stderr of the game to the logger
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

    // This notifies us when the game has actually exited
    game_manager.exit_handle.await.unwrap();
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

    info!("Got token. Returning to process");
    state
        .shutdown_send
        .send(())
        .await
        .expect("To be able to send shutdown signal");
}
