use axum_login::AuthUser;
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
