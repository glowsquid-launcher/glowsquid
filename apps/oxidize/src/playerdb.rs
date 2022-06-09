use serde::*;

use crate::error::AuthError;

#[derive(Debug, Serialize, Deserialize)]
pub struct PlayerDbMinecraftProfile {
  #[serde(rename = "code")]
  pub(crate) code: String,

  #[serde(rename = "message")]
  pub(crate) message: String,

  #[serde(rename = "data")]
  pub(crate) data: Data,

  #[serde(rename = "success")]
  pub(crate) success: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
  #[serde(rename = "player")]
  pub(crate) player: Player,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Player {
  #[serde(rename = "meta")]
  pub(crate) meta: Meta,

  #[serde(rename = "username")]
  pub(crate) username: String,

  #[serde(rename = "id")]
  pub(crate) id: String,

  #[serde(rename = "raw_id")]
  pub(crate) raw_id: String,

  #[serde(rename = "avatar")]
  pub(crate) avatar: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Meta {
  #[serde(rename = "name_history")]
  pub(crate) name_history: Vec<NameHistory>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NameHistory {
  #[serde(rename = "name")]
  pub(crate) name: String,

  #[serde(rename = "changedToAt")]
  pub(crate) changed_to_at: Option<i64>,
}

pub async fn get_profile(uuid: &String) -> Result<PlayerDbMinecraftProfile, AuthError> {
  let url = format!("https://playerdb.co/api/player/minecraft/{}", uuid);

  let resp = reqwest::get(&url)
    .await
    .map_err(|e| AuthError::RequestError(e.into()))?;

  resp
    .json()
    .await
    .map_err(|e| AuthError::RequestError(e.into()))
}
