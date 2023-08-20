use serde::{Deserialize, Serialize};

use crate::assets::client::{
    Arguments, AssetIndex, Downloads, JavaVersion, Library, Logging, Type,
};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct MergableManifest {
    asset_index: Option<AssetIndex>,
    assets: Option<String>,
    downloads: Option<Downloads>,
    id: Option<String>,
    /// Default to be empty
    libraries: Vec<Library>,
    main_class: Option<String>,
    /// Used before 1.13. Already optional.
    minecraft_arguments: Option<String>,
    /// Already optional. When merging, this is preferred over `minecraft_arguments`
    arguments: Option<Arguments>,
    minimum_launcher_version: Option<i64>,
    /// Already optional
    java_version: Option<JavaVersion>,
    release_time: Option<String>,
    time: Option<String>,
    #[serde(rename = "type")]
    manifest_type: Option<Type>,
    /// Already optional
    compliance_level: Option<u8>,
    /// Already optional
    logging: Option<Logging>,
    /// Already optional.
    inherits_from: Option<String>,
}

impl MergableManifest {
    pub fn merge_with(&mut self, other: Self) {
        // arguments (vector merging)
        if let Some(arguments) = other.arguments {
            let mut current_arguments = self.arguments.take().unwrap_or_default();

            current_arguments.jvm.extend(arguments.jvm);
            current_arguments.game.extend(arguments.game);

            self.arguments = Some(current_arguments);
            self.minecraft_arguments = None;
        }

        // inheriting (overriding reverse)
        self.inherits_from = other.inherits_from.or_else(|| self.inherits_from.take());

        // asset index (overriding)
        self.asset_index = self.asset_index.take().or(other.asset_index);

        // compliance (overriding)
        self.compliance_level = self.compliance_level.take().or(other.compliance_level);

        // download (overriding)
        self.downloads = self.downloads.take().or(other.downloads);

        // id (overriding)
        self.id = self.id.take().or(other.id);

        // java version (overriding)
        self.java_version = self.java_version.take().or(other.java_version);

        // library (combining)
        self.libraries.extend(other.libraries);

        // main class (overriding)
        self.main_class = self.main_class.take().or(other.main_class);

        // minimum launcher version (overriding)
        self.minimum_launcher_version = self
            .minimum_launcher_version
            .take()
            .or(other.minimum_launcher_version);

        // release time (overriding)
        self.release_time = self.release_time.take().or(other.release_time);

        // time (overriding)
        self.time = self.time.take().or(other.time);
    }
}
