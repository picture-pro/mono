use leptos::prelude::*;
use models::PhotoGroupConfig;
use reactive_stores::Store;

use super::super::UploadState;
use crate::{
  UploadStateStoreFields, configuring_group::ConfiguringGroupStateStoreFields,
  server_fns::create_photo_group_from_images,
  upload_finished::UploadFinishedState,
};

#[component]
pub(super) fn NextStepButton() -> impl IntoView {
  use lsc::{button::*, icons::*};

  let context: Store<UploadState> = expect_context();
  let state = context
    .configuring_group_0()
    .expect("`UploadContext` not in state `ConfiguringGroup`");
  let photos = state.photos();

  let ready_to_advance = Memo::new(move |_| {
    state.usage_rights_price().get().is_some() && !photos.read().is_empty()
  });

  let disabled_signal = Signal::derive(move || !ready_to_advance());

  let action = Action::new(move |(): &()| {
    let artifact_ids = photos
      .read()
      .values()
      .map(super::uploaded_photo::UploadedPhoto::image_id)
      .collect::<Vec<_>>();
    let usage_rights_price = state
      .usage_rights_price()
      .get()
      .expect("`usage_rights_price` is `None`");
    create_photo_group_from_images(artifact_ids, PhotoGroupConfig {
      usage_rights_price,
    })
  });

  let handler = move |_| {
    action.dispatch(());
  };

  Effect::watch(
    move || action.value().get(),
    move |value, _, _| {
      if let Some(Ok(photo_group_id)) = value {
        *context.write() = UploadState::UploadFinished(UploadFinishedState {
          photo_group: *photo_group_id,
        });
      }
    },
    false,
  );

  view! {
    <Button size=ButtonSize::Large disabled={disabled_signal} {..} on:click=handler>
      "Upload"
      <UploadIcon {..} class="size-6" />
    </Button>
  }
}
