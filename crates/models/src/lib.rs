//! Domain models for the PicturePro project.

mod artifact;
mod photo;

pub use dvf::{slugger::*, *};
pub use model::*;

pub use self::{artifact::*, photo::*};
