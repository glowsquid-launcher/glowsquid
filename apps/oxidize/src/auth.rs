use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MinecraftProfile {
  pub uuid: String,
  pub access_token: String,
  pub refresh_token: String,
}
