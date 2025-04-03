mod home_page;
mod login_page;
mod logout_page;
mod not_found_page;
mod profile_page;
mod protected_page;
mod signup_page;

pub use self::{
  home_page::*, login_page::*, logout_page::*, not_found_page::*,
  profile_page::*, protected_page::*, signup_page::*,
};
