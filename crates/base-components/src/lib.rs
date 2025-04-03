//! Base components for use within PicturePro pages.

#![expect(unexpected_cfgs)]

mod page_container;
mod section;
mod title;

pub use self::{page_container::*, section::*, title::*};
