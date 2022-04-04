use async_graphql::*;

// TODO: Mod versions

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum ModSource {
    Modrinth,
    Curseforge
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
enum SideSupport {
    Required,
    Optional,
    Unsupported
}

#[derive(SimpleObject)]
pub struct ModListing {
    id: ID,
    source: ModSource,
    short_description: String,
    icon_url: String
}

#[derive(SimpleObject)]
pub struct Urls {
    website: Option<String>,
    wiki: Option<String>,
    issues: Option<String>,
    source: Option<String>
}

#[derive(SimpleObject)]
pub struct Picture {
    title: String,
    description: String,
    url: String
}

#[derive(SimpleObject)]
pub struct Mod {
id: ID,
    source: ModSource,
    short_description: String,
    long_description: String,
    download_count: String,
    urls: Urls,
    icon_url: String,
    client_side: SideSupport,
    server_side: SideSupport,
    categories: Vec<String>,
    pictures: Vec<Picture>
}
