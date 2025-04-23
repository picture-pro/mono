use leptos::prelude::*;

#[component]
pub fn Prose(children: Children) -> impl IntoView {
  view! {
    <p class="text-base-dim max-w-prose">
      { children() }
    </p>
  }
}
