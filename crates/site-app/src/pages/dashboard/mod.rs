use leptos::*;

use crate::pages::PageWrapper;

#[component]
pub fn DashboardPage() -> impl IntoView {
  view! {
    <PageWrapper>
      <p class="text-4xl font-semibold tracking-tight">"Dashboard Page"</p>
    </PageWrapper>
  }
}
