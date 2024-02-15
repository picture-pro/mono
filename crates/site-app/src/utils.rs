use leptos::*;

pub fn authenticated_user() -> Option<core_types::User> {
  use_context::<core_types::LoggedInUser>().and_then(|u| u.0)
}
