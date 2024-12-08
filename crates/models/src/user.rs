use dvf::slugger::{EitherSlug, LaxSlug};
use model::{Model, RecordId};
use serde::{Deserialize, Serialize};

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
  pub name:  dvf::HumanName,
  /// The user's email address.
  pub email: dvf::EmailAddress,
  /// The user's authentication secrets.
  pub auth:  UserAuthCredentials,
}

/// The authentication method for a [`User`].
#[derive(Clone, Debug, Hash, PartialEq, Serialize, Deserialize)]
pub enum UserAuthCredentials {
  /// Indicates that the user is authenticated through just an email entry, and
  /// no other verification. VERY DANGEROUS.
  EmailEntryOnly(dvf::EmailAddress),
}

impl Model for User {
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
  pub name:  dvf::HumanName,
  /// The user's email address.
  pub email: dvf::EmailAddress,
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
