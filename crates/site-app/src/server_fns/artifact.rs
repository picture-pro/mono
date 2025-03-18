#[cfg(feature = "ssr")]
mod upload_artifact;

#[cfg(feature = "ssr")]
pub use self::upload_artifact::*;
