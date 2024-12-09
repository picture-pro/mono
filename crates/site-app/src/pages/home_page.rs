use leptos::prelude::*;
use lsc::*;

use crate::components::Section;

#[component]
pub fn HomePage() -> impl IntoView {
  view! {
    <Section>
      <div class="inline-flex flex-col items-start">
        <p class="text-6xl font-serif font-light tracking-tight mb-4">
          "Welcome to "
          <span class="font-semibold">"PicturePro"</span>
          "."
        </p>
        <p class="max-w-prose text-base-dim">
          "Sharing candids is now easier than ever. Cute moment at the park? Done. Posing at the Grand Canyon? Done. "
          <span class="font-semibold">"PicturePro"</span>
          " is the best way to share your photos with friends and family."
        </p>
        <div class="h-8" />
        <div class="mx-0">
          <Button href="/sign-up" color=ButtonColor::Primary size=ButtonSize::Large>
            "Get started"
            <icons::ArrowRightIcon {..} class="size-5" />
          </Button>
        </div>
      </div>
    </Section>
  }
}

#[server]
pub async fn enumerate_photos() -> Result<Vec<models::Photo>, ServerFnError> {
  let service: prime_domain::DynPrimeDomainService = expect_context();

  service.enumerate_photos().await.map_err(ServerFnError::new)
}
