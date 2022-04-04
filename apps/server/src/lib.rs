use async_graphql::EmptyMutation;
use async_graphql::EmptySubscription;
use async_graphql::Schema;
use worker::*;

use serde::Deserialize;

pub mod mods;
mod query;
mod utils;

use query::Query;

#[derive(Deserialize)]
struct GraphQL {
    query: String,
}

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
            Response::ok("POST to /graphql for the API (Use insomnia or something as a client)")?
                .with_cors(
                    &Cors::new()
                        .with_origins(vec!["*"])
                        .with_methods(vec![Method::Get]),
                )
        })
        .post_async("/graphql", |mut req, _| async move {
            let query = req.json::<GraphQL>().await?.query;

            let schema = Schema::new(Query, EmptyMutation, EmptySubscription);
            let res = schema.execute(query).await;

            Response::ok(serde_json::to_string(&res)?)?.with_cors(
                &Cors::new()
                    .with_origins(vec!["*"])
                    .with_methods(vec![Method::Post]),
            )
        })
        .run(req, env)
        .await
}
