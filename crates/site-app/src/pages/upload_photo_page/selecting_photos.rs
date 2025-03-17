use leptos::prelude::*;
use reactive_stores::Store;

use crate::pages::upload_photo_page::UploadStateStoreFields;

#[component]
pub(super) fn SelectingPhotosStep() -> impl IntoView {
  use lsc::button::*;

  let context: Store<super::UploadState> = expect_context();
  let state = context
    .selecting_photos_0()
    .expect("`UploadContext` not in state `SelectingPhotos`");

  view! {
    <p>"Selecting Photos"</p>
    <Button {..} on:click={move |_| {state.ready().set(true);} }>
      "Test"
    </Button>
  }
}

#[derive(Debug, Store)]
pub(super) struct SelectingPhotosState {
  pub ready: bool,
}
