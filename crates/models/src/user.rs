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

/// A password hash.
#[derive(Clone, Debug, Hash, PartialEq, Serialize, Deserialize)]
pub struct PasswordHash(pub String);

/// The user-submitted version of [`UserAuthCredentials`].
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum UserSubmittedAuthCredentials {
  /// Standard email and password.
  EmailAndPassword {
    /// The email used.
    email:    EmailAddress,
    /// The password used.
    password: String,
  },
}

/// The authentication method for a [`User`].
#[derive(Clone, Debug, Hash, PartialEq, Serialize, Deserialize)]
pub enum UserAuthCredentials {
  /// Standard email and password (hash).
  EmailAndPassword {
    /// The email used.
    email:         EmailAddress,
    /// The hash of the password used.
    password_hash: PasswordHash,
  },
}

impl Model for User {
  const INDICES: &'static [(&'static str, model::SlugFieldGetter<Self>)] = &[];
  const TABLE_NAME: &'static str = USER_TABLE_NAME;
  const UNIQUE_INDICES: &'static [(
    &'static str,
    model::SlugFieldGetter<Self>,
  )] = &[("email", |u| EitherSlug::Lax(LaxSlug::new(u.email.as_ref())))];

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

/// A public view of a [`User`], able to be shown to other users.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PublicUser {
  /// The user's ID.
  pub id:   UserRecordId,
  /// The user's name.
  pub name: HumanName,
}

impl From<User> for PublicUser {
  fn from(user: User) -> Self {
    Self {
      id:   user.id,
      name: user.name,
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
