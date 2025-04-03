//! Server functions for use all over the app.

mod artifact;

#[cfg(feature = "ssr")]
pub use self::artifact::*;
