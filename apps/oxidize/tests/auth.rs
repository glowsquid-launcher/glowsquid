use async_trait::async_trait;
use reqwest::Url;
use test_context::{futures, test_context, AsyncTestContext};
use wiremock::{
  matchers::{method, path, query_param},
  Mock, MockServer, ResponseTemplate,
};

use glowsquid::{
  auth::*,
  prisma::{new_client, PrismaClient},
};

/// This is TNT_Man1671's profile (AKA Suyashtnt)
const MINECRAFT_ID: &str = "52ddf2f1-a59f-4a19-822f-a6157f705320";
const DEFAULT_PORT: u16 = 4000;

const REFRESH_TOKEN: &str =
  "dsklfajsdalkfj4213l4JKldsafa1243.0das8g8sddsafSAlfladskfjslad.j1ldfkajdee";
const ACCESS_TOKEN: &str = "am123oNg.u5.dksjfa213Hlksafjlkajg2.jlkkJalfadf";
const EXPIRES_IN: u32 = 3600;

const REFRESHED_REFRESH_TOKEN: &str = "jafkdsafl124.Jkagha241.Jlkh1lk35154";
const REFRESHED_ACCESS_TOKEN: &str = "1dsaklfhafd3.l41s9.lkdsafhsaJ2h41k4h";

async fn add_mock_routes(server: &MockServer) {
  Mock::given(method("POST"))
    .and(path("/api/auth/refresh"))
    .and(query_param("refreshToken", REFRESH_TOKEN))
    .respond_with(ResponseTemplate::new(200).set_body_json(MinecraftProfile {
      uuid: MINECRAFT_ID.to_string(),
      refresh_token: REFRESHED_REFRESH_TOKEN.to_string(),
      access_token: REFRESHED_ACCESS_TOKEN.to_string(),
      expires_in: EXPIRES_IN,
    }))
    .mount(server)
    .await;
}

struct Context {
  server_url: String,
  db: PrismaClient,
}

#[async_trait]
impl AsyncTestContext for Context {
  async fn setup() -> Self {
    let mock_server = MockServer::start().await;
    add_mock_routes(&mock_server).await;
    let db = new_client().await.unwrap();

    Context {
      server_url: mock_server.uri(),
      db,
    }
  }

  async fn teardown(self) {
    // reset the db
    self
      .db
      .account()
      .find_many(vec![])
      .delete()
      .exec()
      .await
      .unwrap();
  }
}

#[test_context(Context)]
#[tokio::test]
async fn can_parse_from_url(ctx: &mut Context) {
  let url = format!(
        "localhost:{}/cb?minecraftId={}&microsoftRefreshToken={}&minecraftToken={}&microsoftExpiresIn={}",
        DEFAULT_PORT, MINECRAFT_ID, REFRESH_TOKEN, ACCESS_TOKEN, EXPIRES_IN
      );

  let test_profile: MinecraftProfile = MinecraftProfile {
    uuid: MINECRAFT_ID.to_string(),
    refresh_token: REFRESH_TOKEN.to_string(),
    access_token: ACCESS_TOKEN.to_string(),
    expires_in: EXPIRES_IN,
  };

  let profile =
    create_profile_from_url(&Url::parse(&url).unwrap()).expect("Could not create profile");

  assert_eq!(test_profile, profile);
}

#[test_context(Context)]
#[tokio::test]
async fn can_authenticate_user_from_url(ctx: &mut Context) {
  let url = format!(
        "localhost:{}/cb?minecraftId={}&microsoftRefreshToken={}&minecraftToken={}&microsoftExpiresIn={}",
        DEFAULT_PORT, MINECRAFT_ID, REFRESH_TOKEN, ACCESS_TOKEN, EXPIRES_IN
      );

  let test_profile: MinecraftProfile = MinecraftProfile {
    uuid: MINECRAFT_ID.to_string(),
    refresh_token: REFRESH_TOKEN.to_string(),
    access_token: ACCESS_TOKEN.to_string(),
    expires_in: EXPIRES_IN,
  };

  process_adding_account(&ctx.db, url)
    .await
    .expect("Could not process adding account");

  let account = get_account(&ctx.db, &MINECRAFT_ID.to_string())
    .await
    .unwrap();

  assert_eq!(test_profile.uuid, account.uuid);
  assert_eq!(test_profile.refresh_token, account.refresh_token);
  assert_eq!(test_profile.access_token, account.access_token);
  assert!(test_profile.expires_in + 2 >= EXPIRES_IN);

  let refreshed_profile = MinecraftProfile {
    uuid: MINECRAFT_ID.to_string(),
    refresh_token: REFRESHED_REFRESH_TOKEN.to_string(),
    access_token: REFRESHED_ACCESS_TOKEN.to_string(),
    expires_in: EXPIRES_IN,
  };

  refresh_account(
    &ctx.db,
    &test_profile.uuid,
    &Url::parse(&format!("{}/api/auth/refresh", &ctx.server_url)).expect("Could not parse url"),
  )
  .await
  .expect("Could not refresh account");

  let refreshed_account = get_account(&new_client().await.unwrap(), &MINECRAFT_ID.to_owned())
    .await
    .unwrap();

  assert_eq!(refreshed_profile.uuid, refreshed_account.uuid);
  assert_eq!(
    refreshed_profile.refresh_token,
    refreshed_account.refresh_token
  );
  assert_eq!(
    refreshed_profile.access_token,
    refreshed_account.access_token
  );
  assert!(refreshed_account.expires_in + 2 >= EXPIRES_IN);
}
