use std::fmt::Debug;

use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Validate, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SignupParams {
  #[validate(
    length(min = 3, message = "Name must be at least 3 characters long"),
    length(max = 256, message = "Name must be at most 256 characters long")
  )]
  pub name:     String,
  #[validate(email(message = "Invalid email address"))]
  pub email:    String,
  #[validate(
    length(min = 8, message = "Password must be at least 8 characters long"),
    length(
      max = 256,
      message = "Password must be at most 256 characters long"
    )
  )]
  pub password: String,
  #[validate(must_match(
    other = "password",
    message = "Passwords do not match"
  ))]
  pub confirm:  String,
}

impl Debug for SignupParams {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("SignupParams")
      .field("name", &self.name)
      .field("email", &self.email)
      .field("password", &"[redacted]")
      .field("confirm", &"[redacted]")
      .finish()
  }
}

#[derive(Validate, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LoginParams {
  pub email:    String,
  pub password: String,
}

impl Debug for LoginParams {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("LoginParams")
      .field("email", &self.email)
      .field("password", &"[redacted]")
      .finish()
  }
}
