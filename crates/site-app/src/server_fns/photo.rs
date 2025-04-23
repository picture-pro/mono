#[cfg(feature = "ssr")]
mod fetch_photo_thumbnail;

#[cfg(feature = "ssr")]
pub use self::fetch_photo_thumbnail::*;
