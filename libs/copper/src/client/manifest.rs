use std::{error::Error, fmt};

use error_stack::{report, Report, ResultExt};

use crate::api::{
    self,
    client::{
        Arguments, AssetIndex, Downloads, GameArgument, JavaVersion, JvmArgument, Logging, Type,
    },
};

use super::library::Library;

/// A launcher manifest with a standardized set of options and API
#[derive(Debug, Clone)]
pub struct Manifest {
    /// The asset index to download
    asset_index: AssetIndex,
    /// The name of the asset index. Useful for saving the asset index to a file
    assets: String,
    /// The versions of the game that are available to download
    downloads: Downloads,
    /// The id of the version
    id: String,
    /// The libraries that are required to run the game
    libraries: Vec<Library>,
    /// The main class to run
    ///
    /// This is the actual class name.
    /// Some were obfuscated by Mojang which requires special handling during creating the [`LauncherManifest`]
    main_class: String,
    /// The arguments to launch the game with
    arguments: Arguments,
    /// The recommended (and minimum) java version required to run the game
    java_version: JavaVersion,
    /// The type of version
    manifest_type: Type,
    /// The logging information for the game
    logging: Logging,
}

impl Manifest {
    #[must_use]
    pub const fn logging(&self) -> &Logging {
        &self.logging
    }

    #[must_use]
    pub const fn manifest_type(&self) -> &Type {
        &self.manifest_type
    }

    #[must_use]
    pub const fn java_version(&self) -> &JavaVersion {
        &self.java_version
    }

    #[must_use]
    pub const fn arguments(&self) -> &Arguments {
        &self.arguments
    }

    #[must_use]
    pub fn libraries(&self) -> &[Library] {
        &self.libraries
    }

    #[must_use]
    pub fn main_class(&self) -> &str {
        &self.main_class
    }

    #[must_use]
    pub const fn downloads(&self) -> &Downloads {
        &self.downloads
    }

    pub fn add_jvm_argument(&mut self, arg: JvmArgument) {
        self.arguments.jvm.push(arg);
    }

    #[must_use]
    pub const fn asset_index(&self) -> &AssetIndex {
        &self.asset_index
    }

    #[must_use]
    pub fn assets(&self) -> &str {
        &self.assets
    }

    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }
}

impl TryFrom<api::client::Manifest> for Manifest {
    type Error = Report<ConversionError>;

    #[allow(clippy::manual_let_else)]
    fn try_from(value: api::client::Manifest) -> Result<Self, Self::Error> {
        let java_version = value.java_version.unwrap_or(JavaVersion {
            component: "java-runtime-delta".to_string(),
            major_version: 8,
        });

        let libraries = value
            .libraries
            .into_iter()
            .map(|library| {
                Library::try_from(library).change_context(ConversionError::LibraryConversionError)
            })
            .collect::<Result<Vec<Library>, Report<ConversionError>>>()?;

        // bug: this cannot be written as let ... else as we're returning something from both branches
        let arguments = match value.arguments {
            Some(args) => args,
            None => Arguments {
                game: value
                    .minecraft_arguments
                    .ok_or(report!(ConversionError::MissingArguments))?
                    .split_whitespace()
                    .map(ToString::to_string)
                    .map(GameArgument::String)
                    .collect(),
                jvm: todo!(
                    "Figure out best default jvm arguments for the game for pre 1.13 versions"
                ),
            },
        };

        Ok(Self {
            asset_index: value.asset_index,
            assets: value.assets,
            downloads: value.downloads,
            id: value.id,
            libraries,
            main_class: value.main_class,
            arguments,
            java_version,
            manifest_type: value.manifest_type,
            logging: value
                .logging
                .unwrap_or_else(|| todo!("Create default logging")),
        })
    }
}

#[derive(Debug)]
pub enum ConversionError {
    LibraryConversionError,
    MissingArguments,
}

impl fmt::Display for ConversionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LibraryConversionError => f.write_str("Failed to convert a library to LauncherLibrary"),
            Self::MissingArguments => f.write_str("Missing arguments field in the manifest. There should either be the modern 1.13 arguments struct or the legacy <1.13 minecraft arguments"),
        }
    }
}

impl Error for ConversionError {}
