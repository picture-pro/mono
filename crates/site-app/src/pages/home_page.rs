use leptos::*;

#[component]
pub fn HomePage() -> impl IntoView {
  view! {
    <super::SmallPageWrapper>
      <p class="text-lg font-semibold tracking-tight">"Welcome to Leptos!"</p>
      <p>"This is a simple example of a Leptos application."</p>
    </super::SmallPageWrapper>
  }
}
