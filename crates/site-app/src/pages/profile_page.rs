use leptos::prelude::*;

use crate::components::Section;

#[component]
pub fn ProfilePage() -> impl IntoView {
  view! {
    <Section>
      <p class="text-6xl font-serif font-light tracking-tight mb-4">
        "User Profile"
      </p>
    </Section>
    <Section>
      <div class="flex flex-row gap-2 justify-between">
        <div class="space-y-4">
          <p class="max-w-prose text-base-dim">"We've got nothing else to display here right now."</p>
          <p class="max-w-prose text-base-dim">"Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua."</p>
        </div>
        <UploadPhotoButton />
      </div>
    </Section>

  }
}

#[component]
pub fn UploadPhotoButton() -> impl IntoView {
  use lsc::{button::*, icons::*};

  view! {
    <Button
      is_link=true
      color=ButtonColor::Primary
      size={ButtonSize::Large}
      {..}
      href="/upload-photo"
    >
      "Upload Photo"
      <UploadIcon {..} class="size-5" />
    </Button>
  }
}
