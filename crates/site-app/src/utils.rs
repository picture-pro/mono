use leptos::*;

#[server]
pub async fn authenticated_user() -> Result<auth_types::User, ServerFnError> {
  let user = use_context::<auth_types::LoggedInUser>()
    .ok_or_else(|| ServerFnError::new(format!("Failed to get auth status")))?;
  let user = user
    .0
    .ok_or_else(|| ServerFnError::new("Unauthenticated"))?;
  Ok(user)
}
