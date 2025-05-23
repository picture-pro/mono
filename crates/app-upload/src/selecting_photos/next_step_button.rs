use std::collections::HashMap;

use leptos::prelude::*;
use reactive_stores::Store;

use crate::{
  UploadState, UploadStateStoreFields,
  configuring_group::{ConfiguringGroupState, UploadedPhoto},
  photo::PhotoUploadStatus,
  selecting_photos::SelectingPhotosStateStoreFields,
};

#[component]
pub(super) fn NextStepButton() -> impl IntoView {
  use lsc::{button::*, icons::*};

  let context: Store<super::super::UploadState> = expect_context();

  let ready_to_advance = Memo::new(move |_| {
    let state = context
      .selecting_photos_0()
      .expect("`UploadContext` not in state `SelectingPhotos`");
    let photos = state.photos();

    if photos.read().is_empty() {
      return false;
    }

    photos
      .read()
      .values()
      .map(|p| p.upload_status()())
      .all(|s| {
        matches!(
          s,
          PhotoUploadStatus::UploadFinished | PhotoUploadStatus::Oversized(_)
        )
      })
  });

  let disabled_signal = Signal::derive(move || !ready_to_advance());

  let handler = move |_| {
    let state = context
      .selecting_photos_0()
      .expect("`UploadContext` not in state `SelectingPhotos`");
    let photos = state.photos();

    let uploaded_photos: HashMap<_, _> = photos
      .read()
      .values()
      .filter_map(UploadedPhoto::from_photo)
      .map(|up| (up.id(), up))
      .collect();

    let new_state = ConfiguringGroupState {
      photos:             uploaded_photos,
      usage_rights_price: None,
    };
    *context.write() = UploadState::ConfiguringGroup(new_state);
  };

  view! {
    <Button size=ButtonSize::Large disabled={disabled_signal} {..} on:click=handler>
      "Advance"
      <ArrowRightIcon {..} class="size-6" />
    </Button>
  }
}
