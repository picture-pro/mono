//! Base components for use within PicturePro pages.

mod page_container;
mod section;
mod title;
pub mod bridge_types {
  //! Bridge types.

  /// The status of user authentication.
  #[derive(Debug, Clone)]
  pub struct AuthStatus(pub Option<models::PublicUser>);
}
pub mod utils;

pub use self::{page_container::*, section::*, title::*};
