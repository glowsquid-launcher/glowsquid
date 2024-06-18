use error_stack::{report, ResultExt};
use serde::{Deserialize, Serialize};
use serde_json::to_string;
use std::{collections::HashMap, path::Path};
use tokio::fs;
use tracing::debug;

use super::client::SaveError;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Assets {
    pub(crate) objects: HashMap<String, Object>,
}

impl Assets {
    /// Saves the manifest to disk.
    ///
    /// # Errors
    /// Returns a [`SaveError`] if the manifest could not be serialized or if an IO error occurred.
    #[tracing::instrument]
    pub async fn save_to_disk(&self, path: &Path) -> error_stack::Result<(), SaveError> {
        debug!("Saving Asset index to disk");
        debug!("Serializing index to JSON");
        let value = to_string(self).change_context(SaveError::SerializeError)?;

        let directory = path.parent().ok_or_else(|| report!(SaveError::IOError))?;

        if !directory.exists() {
            debug!("Creating directory {}", directory.display());
            fs::create_dir_all(directory)
                .await
                .change_context(SaveError::IOError)?;
        }

        debug!("Writing asset index to {}", path.display());
        fs::write(path, value)
            .await
            .change_context(SaveError::IOError)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Object {
    pub(crate) hash: String,
    pub(crate) size: u64,
}
