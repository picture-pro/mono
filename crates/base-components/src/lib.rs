// not useful for component-oriented crates
#![allow(clippy::must_use_candidate)]

//! Base components for use within `PicturePro` pages.

mod image;
mod page_container;
mod photo_group_qr;
mod prose;
mod section;
mod title;
pub mod utils;

pub use self::{
  image::*, page_container::*, photo_group_qr::*, prose::*, section::*,
  title::*,
};
