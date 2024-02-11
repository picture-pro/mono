use leptos::*;

#[component]
pub fn HomePage() -> impl IntoView {
  let user = crate::utils::authenticated_user();

  view! {
    <super::SmallPageWrapper>
      <div class="d-card-body">
        <p class="text-2xl d-card-title">"Welcome to Leptos!"</p>
        <p>"This is a simple example of a Leptos application."</p>

        { match user {
          Some(user) => view! {
            <p>{format!("You are logged in as {} ({})", user.name, user.email)}</p>
          }.into_view(),
          None => view! {
            <p>"You are not logged in. Please login or sign up."</p>
          }.into_view()
        }}
      </div>
    </super::SmallPageWrapper>
  }
}
