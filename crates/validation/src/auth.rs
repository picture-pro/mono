use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Validate, Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
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

#[derive(Validate, Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct LoginParams {
  #[validate(email)]
  pub email:    String,
  #[validate(length(min = 8, max = 256))]
  pub password: String,
}
