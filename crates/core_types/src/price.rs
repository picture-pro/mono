use serde::{Deserialize, Serialize};

/// Price of an asset.
#[derive(
  Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Default, Copy,
)]
pub struct Price(pub f32);
