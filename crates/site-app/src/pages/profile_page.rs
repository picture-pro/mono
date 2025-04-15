use base_components::{Section, Title};
use leptos::prelude::*;
use models::PhotoGroup;

use crate::server_fns::fetch_photo_groups_for_user;

#[component]
pub fn PhotoGroupPreview(pg: PhotoGroup) -> impl IntoView {
  view! {
    <For
      each=move || pg.photos.clone()
      key=move |p| *p
      children=move |p| view! {
        <img src=format!("/api/photo_thumbnail/{p}") />
      }
    />
  }
}

#[component]
pub fn ProfilePhotoGroupPreview() -> impl IntoView {
  let photo_groups =
    Resource::new(move || (), move |_| fetch_photo_groups_for_user());

  let suspended_fn = move || {
    Suspend::new(async move {
      match photo_groups.await {
        Ok(pgs) => view! {
          <For
            each=move || pgs.clone()
            key=move |pg| pg.id
            children=move |pg| view! { <PhotoGroupPreview pg=pg /> }
          />
        }
        .into_any(),
        Err(e) => {
          let e = e.to_string();
          view! { "failed to fetch photo groups: " {e} }.into_any()
        }
      }
    })
  };

  view! {
    <Suspense fallback=move || view! { "Loading..." }>
      { suspended_fn }
    </Suspense>
  }
}

#[component]
pub fn ProfilePage() -> impl IntoView {
  view! {
    <Section>
      <Title>"User Profile"</Title>
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

    <Section>
      <ProfilePhotoGroupPreview />
    </Section>
  }
}

#[component]
pub fn UploadPhotoButton() -> impl IntoView {
  use lsc::{button::*, icons::*};

  view! {
    <Button
      element_type=ButtonElementType::Link
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
