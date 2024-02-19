use serde::{Deserialize, Serialize};

pub const USER_TABLE: &str = "user";

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[cfg_attr(feature = "ssr", serde(from = "crate::ssr::UlidOrThing"))]
pub struct UserRecordId(pub ulid::Ulid);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
  pub id:      UserRecordId,
  pub name:    String,
  pub email:   String,
  pub pw_hash: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PublicUser {
  pub id:    UserRecordId,
  pub name:  String,
  pub email: String,
}

impl From<User> for PublicUser {
  fn from(u: User) -> PublicUser {
    PublicUser {
      id:    u.id,
      name:  u.name,
      email: u.email,
    }
  }
}

#[derive(Clone, Debug)]
pub struct LoggedInUser(pub Option<PublicUser>);

#[cfg(feature = "auth")]
mod auth_traits {
  use axum_login::AuthUser;

  use super::*;

  impl AuthUser for User {
    type Id = ulid::Ulid;

    fn id(&self) -> Self::Id { self.id.0 }
    fn session_auth_hash(&self) -> &[u8] { self.pw_hash.as_bytes() }
  }
}
