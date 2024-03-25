use leptos::*;

use crate::components::basic::Link;

#[component]
pub fn HomePage() -> impl IntoView {
  match crate::utils::authenticated_user() {
    Some(_) => AuthenticatedHomePage().into_view(),
    None => UnauthenticatedHomePage().into_view(),
  }
}

#[component]
pub fn AuthenticatedHomePage() -> impl IntoView {
  // we only call this when the user is authenticated, so unwrap is safe
  let user = crate::utils::authenticated_user().unwrap();

  view! {
    <super::SmallPageWrapper>
      <div class="flex flex-col p-8 gap-4">
        <p class="text-2xl font-semibold tracking-tight">"Welcome to PicturePro!"</p>
        <p>{format!("You are logged in as {} ({})", user.name, user.email)}</p>
      </div>
    </super::SmallPageWrapper>
  }
}

#[component]
pub fn UnauthenticatedHomePage() -> impl IntoView {
  view! {
    <super::SmallPageWrapper>
      <div class="flex flex-col p-8 gap-4">
        <p class="text-2xl font-semibold tracking-tight">"Welcome to PicturePro!"</p>
        <p>
          "You are not logged in."
        </p>
        <p>"Please "
          <Link href="/login".to_string()>login</Link>
          " or "
          <Link href="/signup".to_string()>"sign up"</Link>
          "."
        </p>
      </div>
    </super::SmallPageWrapper>
  }
}
