use leptos::prelude::*;
use models::PhotoGroupRecordId;
use reactive_stores::Store;

use super::UploadStateStoreFields;

#[island]
pub(super) fn UploadFinishedStep() -> impl IntoView {
  let context: Store<super::UploadState> = expect_context();

  let photo_group_id = move || {
    let state = context
      .upload_finished_0()
      .expect("`UploadContext` not in state `UploadFinished`");
    state.photo_group().get().to_string()
  };

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
