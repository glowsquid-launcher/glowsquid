use crate::mods::*;
use async_graphql::*;

pub struct Query;


#[derive(Enum, Copy, Clone, Eq, PartialEq)]
/// An enum of all avalible loaders
enum Loader {
    /// The fabric loader
    Fabric,
    /// The forge loader
    Forge
}

#[derive(InputObject)]
/// Filters for the mod
struct Filters {
    /// The minecraft version this mod should support
    version: String,
    /// The mod name
    name: String,
    /// The loader the mod supports
    loader: Loader,
}

async fn get_modrinth_mods(limit: i32, filters: &Filters) -> Option<Vec<ModListing>> {
    todo!()
}

async fn get_curseforge_mods(limit: i32, filters: &Filters) -> Option<Vec<ModListing>> {
    todo!()
}

#[Object]
impl Query {
    /// is the server up
    async fn is_up(&self) -> bool {
        true
    }

    /// gets a mod list based on the filters
    async fn get_mod_list(&self, limit: i32, filters: Filters) -> Vec<ModListing> {
        let mut mods = vec![];

        // we dont care if one fails, just continue
        if let Some(mut cf_mods) = get_curseforge_mods(limit, &filters).await {
            mods.append(&mut cf_mods);
        }
        if let Some(mut modrinth_mods) = get_modrinth_mods(limit, &filters).await {
            mods.append(&mut modrinth_mods);
        }

        mods
    }

    async fn get_mod(&self, id: ID, source: ModSource) -> Mod {
        todo!()
    }
}
