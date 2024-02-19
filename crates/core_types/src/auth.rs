use serde::{Deserialize, Serialize};

/// The table name for the user table.
pub const USER_TABLE: &str = "user";

/// The record ID for a user.
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[cfg_attr(feature = "ssr", serde(from = "crate::ssr::UlidOrThing"))]
pub struct UserRecordId(pub ulid::Ulid);

/// A user.
///
/// This is the full user record, including the password hash. It's gated behind
/// the `ssr` feature because we don't want to send the password hash to the
/// client. If you need to send user data to the client, use [`PublicUser`]
/// instead.
#[cfg(feature = "ssr")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
  /// The record ID.
  pub id:      UserRecordId,
  /// The user's name.
  pub name:    String,
  /// The user's email.
  pub email:   String,
  /// The user's password hash.
  pub pw_hash: String,
}

/// A user, with the password hash removed.
///
/// This is in the `core_types` crate not because it's a DB model, but because
/// it's common to multiple crates in picturepro. It's used in place of the
/// [`User`] type when we want to send user data to the client.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PublicUser {
  /// The record ID.
  pub id:    UserRecordId,
  /// The user's name.
  pub name:  String,
  /// The user's email.
  pub email: String,
}

#[cfg(feature = "ssr")]
impl From<User> for PublicUser {
  fn from(u: User) -> PublicUser {
    PublicUser {
      id:    u.id,
      name:  u.name,
      email: u.email,
    }
  }
}

/// A logged-in user.
///
/// This is the type provided to the leptos context that's used whenever we need
/// to fetch the user. We don't use the full `AuthSession` type from the `auth`
/// crate because we need this context type to be isomorphic, and `AuthSession`
/// depends on the `axum_login` crate, which would leak all sorts of
/// dependencies into the client bundle.
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
