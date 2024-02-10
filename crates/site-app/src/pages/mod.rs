pub mod home_page;
pub mod login_page;

use leptos::*;

#[component]
pub fn SmallPageWrapper(children: Children) -> impl IntoView {
  view! {
    <div class="flex flex-col justify-center items-center w-full min-h-screen">
      <div class="d-card w-full max-w-sm bg-base-200 rounded-lg shadow-xl">
        {children()}
      </div>
    </div>
  }
}
