use async_graphql::*;

// TODO: Mod versions

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
/// A list of avalible sources
pub enum ModSource {
    /// The mod came from modrinth
    Modrinth,
    /// The mod came from curseforge
    Curseforge,
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
/// An enum that says weather or not a mod supports/requires it on a specific side
/// (server or client)
enum SideSupport {
    /// You have to have the mod on this side
    Required,
    /// You can but dont have to have the mod on this side
    Optional,
    /// You cannot have the mod on this side/it will do nothing when present on the side
    Unsupported,
}

#[derive(SimpleObject)]
/// A basic mod listing
pub struct ModListing {
    /// The ID of the mod
    id: ID,
    /// The source where the mod came from
    source: ModSource,
    /// A short plaintext description of the mod
    short_description: String,
    /// The URL to the icon of this mod
    icon_url: String,
    /// How many downloads this mod has
    downloads: String
}

#[derive(SimpleObject)]
/// A mods various URLs 
pub struct Urls {
    /// The link to the mods main website
    website: Option<String>,
    /// The link to the mods wiki
    wiki: Option<String>,
    /// The link to the mods issue tracker
    issues: Option<String>,
    /// The link to the mods source code
    source: Option<String>,
}

#[derive(SimpleObject)]
/// A picture/screenshot of the mod
pub struct Picture {
    /// The title of the picture
    title: String,
    /// A description of the picture
    description: String,
    /// A URL to the picture
    url: String,
}

#[derive(SimpleObject)]
/// A mod
pub struct Mod {
    /// The ID of the mod
    id: ID,
    /// The source the mod came from
    source: ModSource,
    /// A short plaintext description of the mod
    short_description: String,
    /// A markdown formatted description of the mod
    long_description: String,
    /// The download count for the mod
    download_count: String,
    /// The URLs for the mods items
    urls: Urls, 
    /// The URL to the icon of the mod
    icon_url: String,
    /// Weather this mod is supported or optional on the client
    client_side: SideSupport,
    /// Weather this mod is supported or option on the server
    server_side: SideSupport,
    /// A list of the mods categories
    categories: Vec<String>,
    /// A list of pictures of the mod
    pictures: Vec<Picture>,
    /// A list of IDs of the mods versions  
    versions: Vec<ID>
}
