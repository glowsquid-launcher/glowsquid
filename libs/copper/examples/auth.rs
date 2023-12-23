use std::{future::IntoFuture, net::SocketAddr, sync::OnceLock};

use axum::{
    extract::{Query, State},
    routing::get,
    Router,
};
use copper::auth::{structs::OauthCode, MSauth};
use error_stack::Report;
use oauth2::{CsrfToken, PkceCodeVerifier};
use tokio::{net::TcpListener, task};
use tracing::info;
use tracing_subscriber::{fmt::format::PrettyFields, prelude::*};

extern crate axum;
extern crate copper;
extern crate error_stack;
extern crate tokio;
extern crate tracing;
extern crate tracing_subscriber;

// glowsquids details. If you are going to make your own app, please do not use these.
// these are just for ease of use.
const CLIENT_ID: &str = "2aa32806-92e3-4242-babc-392ac0f0fd30";
const CLIENT_SECRET: &str = "a8F8Q~v58Hfe4nypPkCoiVSCEBG3WKcnYx9uXc-2";

static CSRF_TOKEN: OnceLock<CsrfToken> = OnceLock::new();
static PKCE_VERIFIER: OnceLock<PkceCodeVerifier> = OnceLock::new();

#[derive(Clone)]
struct AppState {
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
    let router = Router::new()
        .route("/code", get(get_code))
        .with_state(AppState {
            oauth: oauth.clone(),
        });

    let socket_addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(&socket_addr).await.unwrap();
    let server = task::spawn(axum::serve(listener, router.into_make_service()).into_future());

    info!("Initializing initialized on port 3000!...");

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

    info!("Authenticated! Token: {:?}", token);
    info!("testing refresh");

    let ms_token = state
        .oauth
        .refresh_ms_access_token(&ms_token)
        .await
        .expect("To be able to refresh token");

    let token = state
        .oauth
        .get_minecraft_token(ms_token.clone())
        .await
        .expect("To be able to get token");

    info!("Authenticated! Token: {:?}", token);

    info!("You can now run ctrl+c to exit the program");
}
