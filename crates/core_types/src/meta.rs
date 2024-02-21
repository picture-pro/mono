use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
/// The metadata for any object.
pub struct ObjectMeta {
  /// The time the object was created at.
  #[serde(with = "chrono::serde::ts_seconds")]
  pub created_at: chrono::DateTime<chrono::Utc>,
}

impl ObjectMeta {
  /// Create a new object meta with the current time.
  pub fn new() -> Self {
    Self {
      created_at: chrono::Utc::now(),
    }
  }
}

impl Default for ObjectMeta {
  fn default() -> Self { Self::new() }
}
