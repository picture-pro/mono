use leptos::prelude::*;

#[component]
pub fn Section(children: Children) -> impl IntoView {
  view! {
    <section class="my-12 space-y-2">
      { children() }
    </section>
  }
}
