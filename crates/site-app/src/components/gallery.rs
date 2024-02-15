use leptos::*;

#[component]
pub fn Gallery() -> impl IntoView {
  let user = crate::authenticated_user();

  let Some(user) = user else {
    return view! {
      <p>"Not Authenticated"</p>
    }
    .into_view();
  };

  view! {}.into_view()
}
