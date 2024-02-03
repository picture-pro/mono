use axum_login::{AuthUser, AuthnBackend, UserId};
use redact::Secret;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Clone, Deserialize, Debug)]
pub struct AuthenticatedUser {
  pub id:  Thing,
  pw_hash: Secret<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct User {
  pub id:      Thing,
  pub name:    String,
  pub email:   String,
  pub pw_hash: Secret<String>,
}

impl From<User> for AuthenticatedUser {
  fn from(user: User) -> Self {
    Self {
      id:      user.id,
      pw_hash: user.pw_hash,
    }
  }
}

impl AuthUser for AuthenticatedUser {
  type Id = Thing;

  fn id(&self) -> Self::Id { self.id.clone() }
  fn session_auth_hash(&self) -> &[u8] {
    self.pw_hash.expose_secret().as_bytes()
  }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Credentials {
  pub email:    String,
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
  type Error = surrealdb::Error;

  async fn authenticate(
    &self,
    credentials: Self::Credentials,
  ) -> Result<Option<Self::User>, Self::Error> {
    let user: Option<User> = (*self.surreal_client)
      .query(
        "SELECT id FROM users WHERE email = $email AND \
         crypto::argon2::compare(password, $password))",
      )
      .bind(("email", &credentials.email))
      .bind(("password", &credentials.password))
      .await?
      .take(0)?;

    Ok(user.map(AuthenticatedUser::from))
  }

  async fn get_user(
    &self,
    user_id: &UserId<Self>,
  ) -> Result<Option<Self::User>, Self::Error> {
    let user: Option<User> = (*self.surreal_client).select(user_id).await?;
    Ok(user.map(AuthenticatedUser::from))
  }
}

pub type AuthSession = axum_login::AuthSession<Backend>;
