use base_components::{PhotoGroupQrCode, Prose, Section};
use leptos::prelude::*;
use models::PhotoGroupRecordId;
use reactive_stores::Store;

use super::UploadStateStoreFields;

#[island]
pub(super) fn UploadFinishedStep() -> impl IntoView {
  let context: Store<super::UploadState> = expect_context();

  let id = Signal::derive(move || {
    let state = context
      .upload_finished_0()
      .expect("`UploadContext` not in state `UploadFinished`");
    state.photo_group().get()
  });

  view! {
    <Section>
      <Prose>"Show this QR code to share your photo group!"</Prose>
    </Section>

    <Section>
      <div class="flex flex-col justify-center items-center h-full w-full">
        { move || view! {
          <PhotoGroupQrCode id=id() {..} class="aspect-square w-full max-w-96 rounded-lg" />
        }}
      </div>
    </Section>
  }
}

#[derive(Debug, Store)]
pub(super) struct UploadFinishedState {
  pub photo_group: PhotoGroupRecordId,
}
