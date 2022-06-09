use glowsquid::playerdb::get_profile;

/// This is TNT_Man1671's profile (AKA Suyashtnt)
const MINECRAFT_ID: &str = "52ddf2f1-a59f-4a19-822f-a6157f705320";

#[tokio::test]
async fn can_get_from_playerdb() {
  get_profile(&MINECRAFT_ID.to_string()).await.unwrap();
}
