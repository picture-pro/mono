mod artifact;
mod auth;
mod photo;
#[cfg(feature = "ssr")]
pub(crate) mod ssr;

#[cfg(feature = "ssr")]
pub use surreal_id::NewId;
pub use ulid::Ulid;

#[cfg(feature = "ssr")]
pub use self::ssr::AsThing;
pub use self::{artifact::*, auth::*, photo::*};
