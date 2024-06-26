use leptos::*;

use crate::pages::PageWrapper;

#[component]
pub fn DashboardPage() -> impl IntoView {
  if crate::authenticated_user().is_none() {
    return view! {
      <PageWrapper>
        <p class="text-4xl font-semibold tracking-tight">"You are not logged in"</p>
        <p class="text-2xl font-semibold tracking-tight">
          <a href="/login" class="underline hover:no-underline">"Login"</a>
          " or "
          <a href="/signup" class="underline hover:no-underline">"Sign Up"</a>
        </p>
      </PageWrapper>
    };
  }

  view! {
    <PageWrapper backed=false>
      <p class="text-4xl font-semibold tracking-tight">"Private Session Photos"</p>
      <div class="flex flex-col lg:flex-row-reverse items-stretch lg:justify-between gap-4 items-start">
        <crate::components::photo_upload::PhotoUpload />
        // <crate::components::gallery::Gallery />
      </div>
    </PageWrapper>
  }
}
