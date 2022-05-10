use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct MinecraftProfile {
  pub uuid: String,
  pub access_token: String,
  pub refresh_token: String,
}
