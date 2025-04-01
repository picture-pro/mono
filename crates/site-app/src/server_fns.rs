//! Server functions for use all over the app.

mod artifact;
mod photo;

#[cfg(feature = "ssr")]
pub use self::artifact::*;
pub use self::photo::*;
