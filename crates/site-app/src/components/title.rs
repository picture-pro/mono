use leptos::prelude::*;

#[component]
pub fn Title(children: Children) -> impl IntoView {
  view! {
    <p class="text-6xl font-serif font-light tracking-tight mb-4">
      { children() }
    </p>
  }
}
