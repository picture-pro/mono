use leptos::prelude::*;
use reactive_stores::Store;

#[component]
pub(super) fn UploadFinishedStep() -> impl IntoView {
  view! {
    <p>"Upload Finished"</p>
  }
}

#[derive(Debug, Store)]
pub(super) struct UploadFinishedState {}
