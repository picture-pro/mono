use axum_login::{
  AuthManagerLayer, AuthManagerLayerBuilder, AuthUser, AuthnBackend, UserId,
};
use color_eyre::eyre::Result;
use serde::{Deserialize, Serialize};
use surrealdb::{engine::remote::ws::Client, sql::Thing};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuthenticatedUser {
  pub id:      Thing,
  pub name:    String,
  pub email:   String,
  pub pw_hash: String,
}

impl From<AuthenticatedUser> for auth_types::User {
  fn from(u: AuthenticatedUser) -> auth_types::User {
    auth_types::User {
      id:    u.id.to_string(),
      name:  u.name,
      email: u.email,
    }
  }
}

impl AuthUser for AuthenticatedUser {
  type Id = Thing;

  fn id(&self) -> Self::Id { self.id.clone() }
  fn session_auth_hash(&self) -> &[u8] { self.pw_hash.as_bytes() }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Credentials {
  pub email:    String,
  pub password: String,
}

#[derive(Clone, Debug)]
pub struct Backend {
  surreal_client: clients::surreal::SurrealRootClient,
}

impl Backend {
  pub async fn new() -> color_eyre::Result<Self> {
    Ok(Self {
      surreal_client: clients::surreal::SurrealRootClient::new().await?,
    })
  }
}

#[async_trait::async_trait]
impl AuthnBackend for Backend {
  type User = AuthenticatedUser;
  type Credentials = Credentials;
  type Error = surrealdb::Error;

  async fn authenticate(
    &self,
    credentials: Self::Credentials,
  ) -> Result<Option<Self::User>, Self::Error> {
    (*self.surreal_client).use_ns("main").await?;
    (*self.surreal_client).use_db("main").await?;

    let user: Option<AuthenticatedUser> = (*self.surreal_client)
      .query(
        "SELECT * FROM users WHERE email = $email AND \
         crypto::argon2::compare(password, $password)",
      )
      .bind(("email", &credentials.email))
      .bind(("password", &credentials.password))
      .await?
      .take(0)?;

    Ok(user)
  }

  async fn get_user(
    &self,
    user_id: &UserId<Self>,
  ) -> Result<Option<Self::User>, Self::Error> {
    let user: Option<AuthenticatedUser> =
      (*self.surreal_client).select(user_id).await?;
    Ok(user)
  }
}

pub type AuthSession = axum_login::AuthSession<Backend>;

/// Builds an authentication layer for use with an Axum router.
pub async fn build_auth_layer() -> Result<
  AuthManagerLayer<
    Backend,
    tower_sessions_surrealdb_store::SurrealSessionStore<Client>,
  >,
> {
  let session_store_surreal_client =
    clients::surreal::SurrealRootClient::new().await?;
  session_store_surreal_client
    .use_ns("main")
    .use_db("main")
    .await?;
  let session_store = tower_sessions_surrealdb_store::SurrealSessionStore::new(
    session_store_surreal_client.into_inner(),
    "user_session".to_string(),
  );
  let session_manager_layer =
    tower_sessions::SessionManagerLayer::new(session_store);

  Ok(
    AuthManagerLayerBuilder::new(Backend::new().await?, session_manager_layer)
      .build(),
  )
}
