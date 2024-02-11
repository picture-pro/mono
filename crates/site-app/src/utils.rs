use leptos::*;

pub fn authenticated_user() -> Option<auth_types::User> {
  use_context::<auth_types::LoggedInUser>()
    .map(|u| u.0)
    .flatten()
}
