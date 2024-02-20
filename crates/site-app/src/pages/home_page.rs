use leptos::*;

use crate::components::basic::Link;

#[component]
pub fn HomePage() -> impl IntoView {
  let user = crate::utils::authenticated_user();

  view! {
    <super::SmallPageWrapper>
      <div class="d-card-body gap-4 w-64">
        <p class="text-2xl font-semibold tracking-tight">"Welcome to PicturePro!"</p>
        { match user {
          Some(user) => view! {
            <p>{format!("You are logged in as {} ({})", user.name, user.email)}</p>
          }.into_view(),
          None => view! {
            <p>
              "You are not logged in."
            </p>
            <p>"Please "
              <Link href="/login".to_string()>login</Link>
              " or "
              <Link href="/signup".to_string()>"sign up"</Link>
              "."
            </p>
          }.into_view()
        }}
      </div>
    </super::SmallPageWrapper>
  }
}
