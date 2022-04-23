use microsoft::{authenticate_user, Code};
use reqwest::StatusCode;
use worker::*;

mod microsoft;
mod utils;

pub const CLIENT_ID: &str = "2aa32806-92e3-4242-babc-392ac0f0fd30";

fn log_request(req: &Request) {
    console_log!("{} - [{}]", Date::now().to_string(), req.path(),);
}

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    log_request(&req);

    // Optionally, get more helpful error messages written to the console in the case of a panic.
    utils::set_panic_hook();

    let router = Router::new();

    router
        .get("/", |_, _| {
            Response::ok("Check out the main website(coming soon:tm:) for api docs")?.with_cors(
                &Cors::new()
                    .with_origins(vec!["*"])
                    .with_methods(vec![Method::Get]),
            )
        })
        .get_async("/microsoft_auth", |req, ctx| async move {
            let url = req.url().unwrap();
            let mut queries = url.query_pairs();

            if let Some((_, code)) = queries.find(|(k, _)| k == "code") {
                let response = authenticate_user(
                    Code::Code(code.to_string()),
                    ctx.var("MS_CLIENT_SECRET")?.to_string(),
                    &url.to_string(),
                )
                .await;

                match response {
                    Ok(value) => Response::from_json(&value),
                    Err(err) => Response::from_json(&err),
                }?
                .with_cors(&Cors::new().with_origins(vec!["*"]))
            } else if let Some((_, refresh_token)) = queries.find(|(k, _)| k == "refresh_token") {
                let response = authenticate_user(
                    Code::RefreshToken(refresh_token.to_string()),
                    ctx.var("MS_CLIENT_SECRET")?.to_string(),
                    &url.to_string(),
                )
                .await;

                match response {
                    Ok(value) => Response::from_json(&value),
                    Err(err) => Response::from_json(&err),
                }?
                .with_cors(&Cors::new().with_origins(vec!["*"]))
            } else {
                Ok(Response::error(
                    "no code or token provided",
                    StatusCode::BAD_REQUEST.as_u16(),
                )?
                .with_cors(&Cors::new().with_origins(vec!["*"]))?)
            }
        })
        .run(req, env)
        .await
}
