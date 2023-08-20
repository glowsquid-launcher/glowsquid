use core::fmt;
use std::{
    error::Error,
    fmt::{Display, Formatter},
};

use error_stack::{IntoReport, ResultExt};
use serde::{Deserialize, Serialize};

use super::client;

const VERSION_MANIFEST_URL: &str =
    "https://piston-meta.mojang.com/mc/game/version_manifest_v2.json";

#[derive(Serialize, Deserialize, Debug)]
pub struct Manifest {
    latest: Latest,
    versions: Vec<Version>,
}

impl Manifest {
    /// Fetches the version manifest from Mojang's servers.
    ///
    /// # Errors
    /// Errors if the request fails or if the response is not a valid [`VersionManifest`].
    pub async fn get() -> Result<Self, reqwest::Error> {
        reqwest::get(VERSION_MANIFEST_URL).await?.json().await
    }

    /// Returns the latest release version.
    ///
    /// # Panics
    /// Panics if the latest snapshot version is not in the manifest. This should never happen.
    #[must_use]
    pub fn latest_release(&self) -> &Version {
        self.versions
            .iter()
            .find(|v| v.id == self.latest.release)
            .expect("Latest version to be in manifest")
    }

    /// Returns the latest snapshot version.
    ///
    /// Note that this may be the same as the latest release version.
    ///
    /// # Panics
    /// Panics if the latest snapshot version is not in the manifest. This should never happen.
    #[must_use]
    pub fn latest_snapshot(&self) -> &Version {
        self.versions
            .iter()
            .find(|v| v.id == self.latest.snapshot)
            .expect("Latest version to be in manifest")
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Latest {
    release: String,
    snapshot: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Version {
    id: String,
    #[serde(rename = "type")]
    version_type: Type,
    url: String,
    time: String,
    release_time: String,
    sha1: String,
    compliance_level: i64,
}

#[derive(Debug)]
pub enum GetError {
    Request,
    CannotParse,
}

impl Display for GetError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Request => write!(f, "Could not get version manifest"),
            Self::CannotParse => write!(
                f,
                "Could not parse version manifest. Please report this as a bug."
            ),
        }
    }
}

impl Error for GetError {}

impl Version {
    /// Tries to download and parse the version manifest.
    ///
    /// # Errors
    /// Errors if the request fails or if the response is not a valid [`client::Manifest`].
    pub async fn download(&self) -> error_stack::Result<client::Manifest, GetError> {
        reqwest::get(&self.url)
            .await
            .into_report()
            .change_context(GetError::Request)?
            .json::<client::Manifest>()
            .await
            .into_report()
            .change_context(GetError::CannotParse)
    }
}

impl Version {
    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Type {
    #[serde(rename = "old_alpha")]
    OldAlpha,
    #[serde(rename = "old_beta")]
    OldBeta,
    Release,
    Snapshot,
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[tokio::test]
    async fn test_getting_latest_release() {
        let manifest = Manifest::get().await.unwrap();

        assert_eq!(manifest.latest_release().version_type, Type::Release);
        // latest snapshot may be the latest release
        assert!(
            manifest.latest_snapshot().version_type == Type::Snapshot
                || manifest.latest_snapshot().version_type == Type::Release
        );
    }

    #[tokio::test]
    async fn parse_latest() {
        let manifest = Manifest::get().await.unwrap();

        manifest.latest_release().download().await.unwrap();
    }

    #[test_case("13w38a"; "Version 1")]
    #[test_case("13w39a"; "Version 2")]
    #[test_case("19w35a"; "Version 3")]
    #[test_case("20w20a"; "Version 4")]
    #[test_case("20w21a"; "Version 5")]
    #[test_case("20w45a"; "Version 6")]
    // some standards
    #[test_case("1.20"; "1.20")]
    #[test_case("1.18"; "1.18")]
    #[test_case("1.13"; "1.13")]
    #[test_case("1.12"; "1.12")]
    #[test_case("1.15"; "1.15")]
    #[test_case("1.7.10"; "1.7.10")]
    #[test_case("1.8.9"; "1.8.9")]
    #[tokio::test]
    async fn parse(id: &str) {
        let manifest = Manifest::get().await.unwrap();

        manifest
            .versions
            .iter()
            .find(|v| v.id == id)
            .unwrap()
            .download()
            .await
            .unwrap();
    }
}
