use serde::{Deserialize, Serialize};

pub const USER_TABLE: &str = "user";

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[cfg_attr(feature = "ssr", serde(from = "crate::conv::UlidOrThing"))]
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

#[cfg(feature = "ssr")]
mod ssr {
  use surreal_id::NewId;
  use surrealdb::sql::Id;

  use super::*;

  impl NewId for UserRecordId {
    const TABLE: &'static str = USER_TABLE;

    fn from_inner_id<T: Into<Id>>(inner_id: T) -> Self {
      Self(inner_id.into().to_string().parse().unwrap())
    }
    fn get_inner_string(&self) -> String { self.0.to_string() }
  }
}

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
