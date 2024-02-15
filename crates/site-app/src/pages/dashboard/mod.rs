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
    <PageWrapper>
      <p class="text-4xl font-semibold tracking-tight">"Dashboard Page"</p>
      <crate::components::photo_upload::PhotoUpload />
    </PageWrapper>
  }
}
