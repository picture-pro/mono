use base_components::{Section, Title};
use leptos::prelude::*;
use lsc::{button::*, icons::*};

#[component]
pub fn HomePage() -> impl IntoView {
  view! {
    <Section>
      <div class="inline-flex flex-col items-start">
        <Title>
          "Welcome to "
          <span class="font-semibold">"PicturePro"</span>
          "."
        </Title>
        <p class="max-w-prose text-base-dim">
          "Sharing candids is now easier than ever. Cute moment at the park? Done. Posing at the Grand Canyon? Easy. "
          <span class="font-semibold">"PicturePro"</span>
          " is the best way to share your photos with friends and family."
        </p>
        <div class="h-8" />
        <Button
          element_type=ButtonElementType::Link color=ButtonColor::Primary
          size={ButtonSize::Large} {..} href="/sign-up" class=("mx-8", move || true)
        >
          "Get started"
          <ArrowRightIcon {..} class="size-5" />
        </Button>
      </div>
    </Section>
  }
}
