use leptos::prelude::*;
use lsc::{button::*, icons::*};

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
          "Sharing candids is now easier than ever. Cute moment at the park? Done. Posing at the Grand Canyon? Easy. "
          <span class="font-semibold">"PicturePro"</span>
          " is the best way to share your photos with friends and family."
        </p>
        <div class="h-8" />
        <Button is_link=true color=ButtonColor::Primary size={ButtonSize::Large} {..} href="/sign-up" class=("mx-8", move || true)>
          "Get started"
          <ArrowRightIcon {..} class="size-5" />
        </Button>
      </div>
    </Section>
  }
}
