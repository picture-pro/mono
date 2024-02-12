pub mod auth;
pub mod dashboard;
pub mod home_page;

use leptos::*;

#[component]
pub fn SmallPageWrapper(children: Children) -> impl IntoView {
  view! {
    <div class="flex-1 flex flex-col justify-center items-center h-full">
      <div class="d-card w-full max-w-sm bg-base-200 rounded-lg shadow-xl">
        {children()}
      </div>
    </div>
  }
}
