use std::fmt::{Display, Formatter};

use async_graphql::*;
use ferinth::structures::version_structs::DependencyType;
use furse::structures::file_structs::FileRelationType;
use serde::Deserialize;

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
/// A list of avalible sources
pub enum ModSource {
    /// The mod came from modrinth
    Modrinth,
    /// The mod came from curseforge
    Curseforge,
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum Loader {
    /// The fabric loader
    Fabric,
    /// The forge loader
    Forge,
}

impl Display for Loader {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Loader::Fabric => write!(f, "fabric"),
            Loader::Forge => write!(f, "forge"),
        }
    }
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum DependencyRelation {
    EmbeddedLibrary,
    OptionalDependency,
    RequiredDependency,
    Tool,
    Incompatible,
    Include,
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
    downloads: String,
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
    versions: Vec<ID>,
}

#[derive(SimpleObject, Clone)]
pub struct Dependency {
    mod_id: ID,
    source: ModSource,
    relation: DependencyRelation,
}

pub struct Version {
    id: ID,
    mod_id: ID,
    source: ModSource,
    name: String,
    game_versions: Vec<String>,
    loader: Loader,
    /// A list of top-level dependencies
    dependencies: Vec<Dependency>,
}

#[Object]
impl Version {
    async fn id(&self) -> ID {
        self.id.clone()
    }
    async fn mod_id(&self) -> ID {
        self.mod_id.clone()
    }
    async fn source(&self) -> ModSource {
        self.source
    }
    async fn name(&self) -> String {
        self.name.clone()
    }
    async fn game_versions(&self) -> Vec<String> {
        self.game_versions.clone()
    }

    /// gets a list of all dependencies, including nested dependencies
    async fn all_dependencies<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        minecraft_version: String,
    ) -> Vec<Dependency> {
        match self.source {
            ModSource::Modrinth => {
                get_deps_modrinth(self.dependencies.clone(), minecraft_version, self.loader).await
            }
            ModSource::Curseforge => {
                get_deps_curseforge(
                    self.dependencies.clone(),
                    minecraft_version,
                    self.loader,
                    ctx.data::<String>().unwrap().to_string(),
                )
                .await
            }
        }
    }
}

#[derive(Deserialize)]
struct GetFilesResponse {
    data: Vec<furse::structures::file_structs::File>,
}

#[async_recursion::async_recursion]
async fn get_deps_curseforge(
    dependencies: Vec<Dependency>,
    minecraft_version: String,
    loader: Loader,
    cf_api_key: String,
) -> Vec<Dependency> {
    let mut deps = dependencies.clone();
    let client = reqwest::Client::new();
    for dep in &dependencies {
        let versions = client
            .get(format!(
                "https://api.curseforge.com/v1/mods/{}/files?gameVersion={}&modLoaderType={}",
                dep.mod_id.to_string(),
                minecraft_version,
                loader.to_string()
            ))
            .header("Accept", "application/json")
            .header("x-api-key", &cf_api_key)
            .send()
            .await
            .unwrap()
            .json::<GetFilesResponse>()
            .await
            .unwrap()
            .data;

        if let Some(version) = versions.iter().next() {
            deps.extend(
                version
                    .dependencies
                    .iter()
                    .map(|dep| Dependency {
                        mod_id: ID(dep.mod_id.to_string()),
                        source: ModSource::Curseforge,
                        relation: match dep.relation_type {
                            FileRelationType::EmbeddedLibrary => {
                                DependencyRelation::EmbeddedLibrary
                            }
                            FileRelationType::OptionalDependency => {
                                DependencyRelation::OptionalDependency
                            }
                            FileRelationType::RequiredDependency => {
                                DependencyRelation::RequiredDependency
                            }
                            FileRelationType::Tool => DependencyRelation::Tool,
                            FileRelationType::Incompatible => DependencyRelation::Incompatible,
                            FileRelationType::Include => DependencyRelation::Include,
                        },
                    })
                    .filter(|dep| !deps.iter().any(|d| d.mod_id == dep.mod_id))
                    .collect::<Vec<_>>(),
            );
        }
    }
    deps
}

#[async_recursion::async_recursion]
async fn get_deps_modrinth(
    dependencies: Vec<Dependency>,
    minecraft_version: String,
    loader: Loader,
) -> Vec<Dependency> {
    let mut deps = dependencies.clone();
    let client = reqwest::Client::new();
    for dep in &dependencies {
        let versions =  client.get(format!(
                            "https://api.modrinth.com/v2/project/{}/version?loaders=[\"{}\"]&game_versions=[\"{}\"]",
                            dep.mod_id.to_string(),
                            &loader,
                            &minecraft_version
                        ))
                            .send()
                            .await
                            .unwrap()
                            .json::<Vec<ferinth::structures::version_structs::Version>>()
                            .await
                            .unwrap();

        if let Some(version) = versions.into_iter().next() {
            deps.extend(
                version
                    .dependencies
                    .iter()
                    .map(|dep| Dependency {
                        mod_id: ID(dep.project_id.clone().unwrap()),
                        source: ModSource::Modrinth,
                        relation: match dep.dependency_type {
                            DependencyType::Required => DependencyRelation::RequiredDependency,
                            DependencyType::Optional => DependencyRelation::OptionalDependency,
                            DependencyType::Incompatible => DependencyRelation::Incompatible,
                        },
                    })
                    .filter(|dep| !deps.iter().any(|dep2| dep2.mod_id == dep.mod_id))
                    .collect::<Vec<_>>(),
            );

            deps.extend(get_deps_modrinth(deps.clone(), minecraft_version.clone(), loader).await);
        }
    }
    deps
}
