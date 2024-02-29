use std::fmt::Debug;

use nutype::nutype;
use serde::{Deserialize, Serialize};

use crate::{NewType, NewTypeError};

macro_rules! impl_newtype {
  ($name:ident, $error:ident) => {
    impl NewType for $name {
      type Inner = String;
      type Error = $error;

      fn new(inner: Self::Inner) -> Result<Self, Self::Error> {
        $name::new(inner)
      }
      fn into_inner(self) -> Self::Inner { self.into_inner() }
    }
  };
}

/// The name of a user.
#[nutype(
  sanitize(trim),
  validate(len_char_min = 3, len_char_max = 256),
  derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)
)]
pub struct Name(String);

impl_newtype!(Name, NameError);

impl NewTypeError for NameError {
  fn to_string(&self) -> String {
    match self {
      __nutype_private_Name__::NameError::LenCharMinViolated => {
        "Name must be at least 3 characters long".to_owned()
      }
      __nutype_private_Name__::NameError::LenCharMaxViolated => {
        "Name must be at most 256 characters long".to_owned()
      }
    }
  }
}

/// An email address.
#[nutype(
  sanitize(trim),
  validate(len_char_max = 256),
  derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)
)]
pub struct Email(String);

impl_newtype!(Email, EmailError);

impl NewTypeError for EmailError {
  fn to_string(&self) -> String {
    match self {
      __nutype_private_Email__::EmailError::LenCharMaxViolated => {
        "Email must be at most 256 characters long".to_owned()
      }
    }
  }
}

/// A password.
#[nutype(
  sanitize(trim),
  validate(len_char_min = 8, len_char_max = 256),
  derive(Serialize, Deserialize, PartialEq, Eq, Clone)
)]
pub struct Password(String);

impl_newtype!(Password, PasswordError);

impl Debug for Password {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str("[redacted]")
  }
}

impl NewTypeError for PasswordError {
  fn to_string(&self) -> String {
    match self {
      __nutype_private_Password__::PasswordError::LenCharMinViolated => {
        "Password must be at least 8 characters long".to_owned()
      }
      __nutype_private_Password__::PasswordError::LenCharMaxViolated => {
        "Password must be at most 256 characters long".to_owned()
      }
    }
  }
}

/// Parameters for signing up.
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct SignupParams {
  /// The user's name.
  pub name:     Name,
  /// The user's email.
  pub email:    Email,
  /// The user's password.
  pub password: Password,
}

/// Parameters for logging in.
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct LoginParams {
  /// The user's email.
  pub email:    Email,
  /// The user's password.
  pub password: Password,
}
