use leptos::prelude::*;
use models::PhotoGroupRecordId;
use reactive_stores::Store;

use super::UploadStateStoreFields;

#[island]
pub(super) fn UploadFinishedStep() -> impl IntoView {
  let context: Store<super::UploadState> = expect_context();
  let state = context
    .upload_finished_0()
    .expect("`UploadContext` not in state `UploadFinished`");
  let photo_group_id = state.photo_group();
  let photo_group_id = Signal::derive(move || photo_group_id.get().to_string());

  view! {
    <p>"Upload Finished"</p>
    <p>
      "Photo Group: "
      { photo_group_id }
    </p>
  }
}

#[derive(Debug, Store)]
pub(super) struct UploadFinishedState {
  pub photo_group: PhotoGroupRecordId,
}
