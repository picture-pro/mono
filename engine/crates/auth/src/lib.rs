use axum_login::{AuthUser, AuthnBackend, UserId};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Clone, Serialize, Deserialize)]
pub struct AuthenticatedUser {
  pub id:       Thing,
  pub username: String,
  pub password: String,
}

impl std::fmt::Debug for AuthenticatedUser {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("AuthenticatedUser")
      .field("id", &self.id)
      .field("username", &self.username)
      .field("password", &"[redacted]")
      .finish()
  }
}

impl AuthUser for AuthenticatedUser {
  type Id = Thing;

  fn id(&self) -> Self::Id { self.id.clone() }
  fn session_auth_hash(&self) -> &[u8] { self.password.as_bytes() }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Credentials {
  pub username: String,
  pub password: String,
  pub next:     Option<String>,
}

#[derive(Clone)]
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
  type Error = std::convert::Infallible;

  async fn authenticate(
    &self,
    credentials: Self::Credentials,
  ) -> Result<Option<Self::User>, Self::Error> {
    let surreal_client = &self.surreal_client;

    Ok(None)
  }

  async fn get_user(
    &self,
    user_id: &UserId<Self>,
  ) -> Result<Option<Self::User>, Self::Error> {
    Ok(None)
  }
}
