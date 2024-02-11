use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
  pub id:    String,
  pub name:  String,
  pub email: String,
}

#[derive(Clone, Debug)]
pub struct LoggedInUser(pub Option<User>);
