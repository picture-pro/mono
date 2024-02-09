use leptos::*;

pub fn auth() -> Option<auth::AuthSession> {
  use_context::<auth::AuthSession>()
}
