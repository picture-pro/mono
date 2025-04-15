//! Server functions for use all over the app.

mod artifact;
mod photo;
mod photo_group;

#[cfg(feature = "ssr")]
pub use self::artifact::*;
pub use self::{photo::*, photo_group::*};
