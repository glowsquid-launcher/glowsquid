use serde_json::json;
use worker::*;

mod utils;

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
        .get("/", |_, _| Response::ok("Hello from Workers!")?.with_cors(&Cors::new().with_origins(vec!["*"]).with_methods(vec![Method::Get])))
        .run(req, env)
        .await
}
