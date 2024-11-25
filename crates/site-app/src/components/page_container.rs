use leptos::prelude::*;

#[component]
pub fn PageContainer(children: Children) -> impl IntoView {
  view! {
    <main class="container mx-auto my-4">
      { children() }
    </main>
  }
}
