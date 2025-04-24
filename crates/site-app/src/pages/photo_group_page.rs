use std::str::FromStr;

use base_components::{Section, Title};
use leptos::{either::Either, prelude::*};
use leptos_router::hooks::use_params_map;
use models::PhotoGroupRecordId;

use crate::pages::NotFoundPage;

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
    Some(id) => Either::Left(view! { <PhotoGroupPageInner id=id /> }),
    None => Either::Right(view! { <NotFoundPage /> }),
  }
}

#[component]
pub fn PhotoGroupPageInner(id: PhotoGroupRecordId) -> impl IntoView {
  view! {
    <Section>
      <Title>"Photo Group"</Title>
      <pre><code>"id = \"" { id.to_string() } "\""</code></pre>
    </Section>
  }
}
