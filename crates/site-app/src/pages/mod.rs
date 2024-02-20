pub mod auth;
pub mod dashboard;
pub mod home_page;

use leptos::*;

#[component]
pub fn SmallPageWrapper(children: Children) -> impl IntoView {
  view! {
    <div class="flex-1 flex flex-col justify-center items-center h-full">
      <div class="d-card w-full max-w-sm bg-base-100 rounded-lg shadow-xl">
        {children()}
      </div>
    </div>
  }
}

#[component]
pub fn PageWrapper(
  children: Children,
  #[prop(default = "bg-base-100")] bg_color: &'static str,
  #[prop(default = "shadow")] shadow: &'static str,
) -> impl IntoView {
  view! {
    <div class={format!("flex-1 flex flex-col gap-4 md:container md:mx-auto mt-8 mb-0 md:my-8 md:rounded-xl {shadow} p-4 md:px-6 {bg_color}")}>
      {children()}
    </div>
  }
}
