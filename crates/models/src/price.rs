use std::fmt;

use serde::{Deserialize, Serialize};

/// A USD price.
#[derive(
  Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Serialize, Deserialize,
)]
pub struct UsdPriceNaive {
  cents: u64,
}

impl UsdPriceNaive {
  /// Creates a new price from an `f32` representing dollars.
  #[must_use]
  #[allow(clippy::cast_possible_truncation)]
  pub const fn new_from_f32(value: f32) -> Self {
    Self {
      cents: ((value.max(0.0) * 100.0) as i64).unsigned_abs(),
    }
  }
  /// Represents a price as dollars.
  #[must_use]
  #[allow(clippy::cast_precision_loss)]
  pub const fn as_f32(&self) -> f32 { self.cents as f32 / 100.0 }
}

// uses the display impl
impl fmt::Debug for UsdPriceNaive {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_tuple("UsdPriceNaive")
      .field(&self.to_string())
      .finish()
  }
}

impl fmt::Display for UsdPriceNaive {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "${:.2}", self.as_f32())
  }
}
