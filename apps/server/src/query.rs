use async_graphql::*;
use crate::mods::*;

pub struct Query;

#[Object]
impl Query {
    /// is the server up
    async fn is_up(&self) -> bool {
        true
    }

    async fn get_mod_list(&self, limit: i32) -> Vec<ModListing> {
        todo!()
    }

    async fn get_mod(&self, id: ID, source: ModSource) -> Mod {
        todo!()
    }
}
