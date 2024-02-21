use leptos::*;
use leptos_router::use_params_map;

use crate::pages::PageWrapper;

#[component]
pub fn PurchasePage() -> impl IntoView {
  let params = use_params_map();
  let id = params().get("id").cloned();

  if id.is_none() || id.unwrap().is_empty() {
    return PurchasePageNoId().into_view();
  }

  view! {
    <PageWrapper>
      <h1 class="text-4xl font-semibold tracking-tight">Purchase</h1>
      <p class="text-lg">
        "This is the purchase page."
      </p>
    </PageWrapper>
  }
}

#[component]
pub fn PurchasePageNoId() -> impl IntoView {
  view! {
    <PageWrapper>
      <h1 class="text-4xl font-semibold tracking-tight">Purchase</h1>
      <p class="text-lg">
        "No photo ID was provided. Perhaps you followed a bad link?"
      </p>
    </PageWrapper>
  }
}
