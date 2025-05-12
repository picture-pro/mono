use std::str::FromStr;

use base_components::{PhotoGroupQrCode, Section, Title};
use leptos::{either::Either, prelude::*};
use leptos_router::hooks::use_params_map;
use models::{PhotoGroupFullQuery, PhotoGroupRecordId};

use crate::{
  components::PhotoPreview, pages::NotFoundPage, server_fns::fetch_photo_group,
};

fn extract_photo_group_id() -> Signal<Option<PhotoGroupRecordId>> {
  let params = use_params_map();
  Signal::derive(move || {
    let params_guard = params.read();
    params_guard
      .get("id")
      .and_then(|s| PhotoGroupRecordId::from_str(&s).ok())
  })
}

#[component]
pub fn PhotoGroupPage() -> impl IntoView {
  let id = extract_photo_group_id();

  move || match id() {
    Some(id) => Either::Left(view! { <PhotoGroupFetcher id=id /> }),
    None => Either::Right(view! { <NotFoundPage /> }),
  }
}

#[component]
pub fn PhotoGroupFetcher(id: PhotoGroupRecordId) -> impl IntoView {
  let pg_resource = Resource::new(move || id, fetch_photo_group);

  let suspended_fn = move || {
    Suspend::new(async move {
      match pg_resource.await {
        Ok(Some(pg)) => view! { <PhotoGroupPageInner pgq=pg />}.into_any(),
        Ok(None) => view! { <NotFoundPage /> }.into_any(),
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
pub fn PhotoGroupPageInner(pgq: PhotoGroupFullQuery) -> impl IntoView {
  let name = pgq.vendor_data.name.as_ref().to_owned();

  view! {
    <Section>
      <Title>
        "Photos by " { name }
      </Title>
    </Section>
    <Section>
      <PhotoGroupDetails pgq=pgq />
    </Section>
  }
}

#[component]
pub fn PhotoGroupDetails(pgq: PhotoGroupFullQuery) -> impl IntoView {
  use lsc::{button::*, icons::*};

  let photo_previews = pgq
    .photo_group
    .photos
    .into_iter()
    .map(|p| view! { <PhotoPreview id=p /> })
    .collect_view();

  view! {
    <div class="flex flex-col-reverse sm:flex-row gap-8 sm:items-start">
      <div class="flex-1 flex flex-row flex-wrap gap-4">
        { photo_previews }
      </div>

      // <div class="my-4 w-[1px] border-l-2 border-dashed border-base-8 dark:border-basedark-8" />

      <div class="sm:min-w-64 flex flex-col gap-6 p-2 pb-8 sm:p-0 sm:pl-4 border-b-2 sm:border-b-0 sm:border-l-2 border-base-8 dark:border-basedark-8">
        <p class="text-3xl">
          "Price: "
          <span class="font-bold">
            { pgq.photo_group.config.usage_rights_price.to_string() }
          </span>
        </p>

        <div class="flex flex-col gap-2">
          <Button
            color=ButtonColor::Primary size=ButtonSize::Large
          >
            "Buy Now"
            <LightningBoltIcon {..} class="size-5" />
          </Button>
          <Button
            color=ButtonColor::Base size=ButtonSize::Large
          >
            "Save For Later"
            <ClockIcon {..} class="size-5" />
          </Button>
        </div>

        <div class="flex flex-col gap-1">
          <p class="text-3xl">"Share:"</p>
          <PhotoGroupQrCode
            id=pgq.photo_group.id {..}
            class="aspect-square w-full max-w-96 rounded-lg"
          />
        </div>
      </div>
    </div>
  }
}
