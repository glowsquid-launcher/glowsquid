use async_graphql::*;

pub struct Query;

#[Object]
impl Query {
    /// is the server up
    async fn is_up(&self) -> bool {
        true
    }
}
