use std::hash::{self, Hash, Hasher};

use model::{Model, RecordId};
use serde::{Deserialize, Serialize};

pub use self::bridge::*;
use crate::{EitherSlug, EmailAddress, HumanName, LaxSlug};

/// The table name for [`User`] records.
pub const USER_TABLE_NAME: &str = "user";

/// An alias for [`RecordId<User>`].
pub type UserRecordId = RecordId<User>;

/// The domain model for a user on the platform.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct User {
  /// The user's ID.
  pub id:    UserRecordId,
  /// The user's name.
  pub name:  HumanName,
  /// The user's email address.
  pub email: EmailAddress,
  /// The user's authentication secrets.
  pub auth:  UserAuthCredentials,
}

impl User {
  /// Returns the hash of the user's authentication secrets.
  #[must_use]
  pub fn auth_hash(&self) -> u64 {
    let mut hasher = hash::DefaultHasher::new();
    self.auth.hash(&mut hasher);
    hasher.finish()
  }
}

/// The authentication method for a [`User`].
#[derive(Clone, Debug, Hash, PartialEq, Serialize, Deserialize)]
pub enum UserAuthCredentials {
  /// Indicates that the user is authenticated through just an email entry, and
  /// no other verification. VERY DANGEROUS.
  EmailEntryOnly(EmailAddress),
}

impl Model for User {
  const TABLE_NAME: &'static str = USER_TABLE_NAME;
  const UNIQUE_INDICES: &'static [(
    &'static str,
    model::SlugFieldGetter<Self>,
  )] = &[("email", |u| EitherSlug::Lax(LaxSlug::new(u.email.as_ref())))];
  const INDICES: &'static [(&'static str, model::SlugFieldGetter<Self>)] = &[];

  fn id(&self) -> UserRecordId { self.id }
}

/// A request to create a new [`User`].
#[derive(Debug)]
pub struct UserCreateRequest {
  /// The user's name.
  pub name:  HumanName,
  /// The user's email address.
  pub email: EmailAddress,
  /// The user's authentication secrets.
  pub auth:  UserAuthCredentials,
}

impl From<UserCreateRequest> for User {
  fn from(req: UserCreateRequest) -> Self {
    Self {
      id:    UserRecordId::new(),
      name:  req.name,
      email: req.email,
      auth:  req.auth,
    }
  }
}

/// An auth-centric view of a [`User`], able to be sent to the client.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthUser {
  /// The user's ID.
  pub id:              UserRecordId,
  /// The user's name.
  pub name:            HumanName,
  /// The hash of the user's authentication secrets.
  pub auth_hash_bytes: Box<[u8]>,
}

impl From<User> for AuthUser {
  fn from(user: User) -> Self {
    let auth_hash_bytes =
      user.auth_hash().to_be_bytes().to_vec().into_boxed_slice();
    Self {
      id: user.id,
      name: user.name,
      auth_hash_bytes,
    }
  }
}

/// Types in this module are for repackaging data to transfer from the high-deps
/// side (server code) to the low-deps side (client code)
mod bridge {
  /// The status of user authentication.
  #[derive(Debug, Clone)]
  pub struct AuthStatus(pub Option<super::AuthUser>);
}

#[cfg(feature = "auth")]
mod auth {
  use axum_login::AuthUser as AxumLoginAuthUser;

  use super::AuthUser;

  impl AxumLoginAuthUser for AuthUser {
    type Id = super::UserRecordId;
    fn id(&self) -> Self::Id { self.id }
    fn session_auth_hash(&self) -> &[u8] { &self.auth_hash_bytes }
  }
}
