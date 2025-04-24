use base_components::{Prose, Section, SmallImage, Title};
use leptos::prelude::*;
use models::PhotoGroup;

use crate::server_fns::fetch_photo_groups_for_user;

#[component]
pub fn PhotoGroupPreview(pg: PhotoGroup) -> impl IntoView {
  use lsc::link::*;

  let class = "bg-base-2 dark:bg-basedark-2 border border-base-7 \
               dark:border-basedark-7 rounded-lg flex flex-col p-4 gap-4 \
               shadow-md";

  let price = format!("Price: {}", pg.config.usage_rights_price);
  let url = format!("/photo-group/{}", pg.id);

  view! {
    <div class=class>
      <Link
        color=LinkColor::Base size=LinkSize::ExtraLarge
        underline=LinkUnderline::Always high_contrast=true
        {..} href=url
      >{ price }</Link>
      <div class="flex flex-row flex-wrap gap-4">
      <For
        each=move || pg.photos.clone()
        key=move |p| *p
        children=move |p| view! {
          <SmallImage url=format!("/api/photo_thumbnail/{p}") />
        }
      />
      </div>
    </div>
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
          <div class="flex flex-col gap-4">
            <For
              each=move || pgs.clone()
              key=move |pg| pg.id
              children=move |pg| view! { <PhotoGroupPreview pg=pg /> }
            />
          </div>
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
          <Prose>"We've got nothing else to display here right now."</Prose>
          <Prose>"Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua."</Prose>
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
