use std::str::FromStr;

use base_components::{Section, SmallImage, Title};
use leptos::{either::Either, prelude::*};
use leptos_router::hooks::use_params_map;
use models::{PhotoGroup, PhotoGroupRecordId};

use crate::{pages::NotFoundPage, server_fns::fetch_photo_group};

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
        Ok(Some(pg)) => view! { <PhotoGroupPageInner pg=pg />}.into_any(),
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
pub fn PhotoGroupPageInner(pg: PhotoGroup) -> impl IntoView {
  view! {
    <Section>
      <Title>"Photo Group"</Title>
    </Section>
    <Section>
      <PhotoGroupDetails pg=pg />
    </Section>
  }
}

#[component]
pub fn PhotoGroupDetails(pg: PhotoGroup) -> impl IntoView {
  use lsc::{button::*, icons::*};

  view! {
    <div class="flex flex-row gap-4 items-start">
      <div class="flex flex-row flex-wrap gap-4">
        <For
          each=move || pg.photos.clone()
          key=move |p| *p
          children=move |p| view! {
            <SmallImage url=format!("/api/photo_thumbnail/{p}") />
          }
        />
      </div>

      // <div class="my-4 w-[1px] border-l-2 border-dashed border-base-8 dark:border-basedark-8" />

      <div class="min-w-64 flex flex-col gap-4 pl-4 border-l-2 border-base-8 dark:border-basedark-8">
        <p class="text-3xl">
          "Price: "
          <span class="font-bold">
            { pg.config.usage_rights_price.to_string() }
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
      </div>
    </div>
  }
}
