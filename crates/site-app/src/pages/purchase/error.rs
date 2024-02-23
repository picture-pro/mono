use leptos::*;

use crate::pages::PageWrapper;

#[component]
pub fn PurchasePageNoId() -> impl IntoView {
  view! {
    <PurchasePageError
      error="No photo ID provided. Perhaps you followed a bad link?".to_string()
    />
  }
}

#[component]
pub fn PurchasePageInvalidId() -> impl IntoView {
  view! {
    <PurchasePageError
      error="The photo ID provided is invalid. Perhaps you followed a bad link?".to_string()
    />
  }
}

#[component]
pub fn PurchasePageMissing() -> impl IntoView {
  view! {
    <PurchasePageError
      error="The photo you are trying to purchase does not exist. Perhaps you followed a bad link?".to_string()
    />
  }
}

#[component]
pub fn PurchasePageInternalError(error: String) -> impl IntoView {
  view! {
    <PurchasePageError
      error="An internal error occurred while trying to fetch the photo you are trying to purchase. Please try again later.".to_string()
    />
  }
}

#[component]
pub fn PurchasePageError(error: String) -> impl IntoView {
  view! {
    <PageWrapper>
      <h1 class="text-4xl font-semibold tracking-tight">Purchase</h1>
      <p class="text-lg"> { error } </p>
    </PageWrapper>
  }
}
