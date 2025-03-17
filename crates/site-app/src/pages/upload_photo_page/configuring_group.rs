use leptos::prelude::*;
use reactive_stores::Store;

use crate::pages::upload_photo_page::UploadStateStoreFields;

#[component]
pub(super) fn ConfiguringGroupStep() -> impl IntoView {
  use lsc::button::*;

  let context: Store<super::UploadState> = expect_context();
  let state = context
    .configuring_group_0()
    .expect("`UploadContext` not in state `ConfiguringGroup`");

  view! {
    <p>"Configuring Group"</p>
    <Button {..} on:click={move |_| {state.ready().set(true);} }>
      "Test"
    </Button>
  }
}

#[derive(Debug, Store)]
pub(super) struct ConfiguringGroupState {
  pub ready: bool,
}
