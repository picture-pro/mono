use std::fmt;

use gloo::file::{Blob, File, ObjectUrl};
use leptos::prelude::{Action, LocalStorage, Signal};
use models::{ArtifactRecordId, FileSize, Ulid};
use reactive_stores::Store;
use send_wrapper::SendWrapper;

use super::MAX_UPLOAD_SIZE;

#[derive(Store)]
pub struct Photo {
  id:           Ulid,
  blob:         SendWrapper<Blob>,
  url:          SendWrapper<ObjectUrl>,
  action_state: PhotoActionState,
}

impl Photo {
  pub(super) fn new(file: File) -> Self {
    let id = Ulid::new();

    let blob = Blob::from(file);
    let url = ObjectUrl::from(blob.clone());
    let blob = SendWrapper::new(blob);
    let action_state = PhotoActionState::new(&blob);

    Self {
      id,
      blob,
      url: SendWrapper::new(url),
      action_state,
    }
  }
  pub(super) fn id(&self) -> Ulid { self.id }
  pub(super) fn blob(&self) -> Blob { self.blob.clone().take() }
  pub(super) fn url(&self) -> ObjectUrl { self.url.clone().take() }
  pub(super) fn oversized(&self) -> Signal<Option<FileSize>> {
    let upload_status = self.upload_status();
    Signal::derive(move || match upload_status() {
      PhotoUploadStatus::Oversized(file_size) => Some(file_size),
      _ => None,
    })
  }
  pub(super) fn upload_status(&self) -> Signal<PhotoUploadStatus> {
    self.action_state.status()
  }
  pub(super) fn artifact_id(&self) -> Signal<Option<ArtifactRecordId>> {
    match self.action_state {
      PhotoActionState::Started(action) => {
        let value = action.value();
        Signal::derive(move || value().and_then(|r| r.ok()))
      }
      PhotoActionState::Oversized(_) => Signal::stored(None),
    }
  }
}

impl fmt::Debug for Photo {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct(stringify!(Photo))
      .field("id", &self.id)
      .field("blob", &self.blob)
      .field("url", &self.url.to_string())
      .field("action_state", &self.action_state)
      .finish()
  }
}

#[derive(Clone)]
pub enum PhotoActionState {
  Started(
    Action<SendWrapper<Blob>, Result<ArtifactRecordId, String>, LocalStorage>,
  ),
  Oversized(FileSize),
}

impl fmt::Debug for PhotoActionState {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::Started(_) => f.debug_tuple("Started").finish(),
      Self::Oversized(size) => f.debug_tuple("Oversized").field(size).finish(),
    }
  }
}

impl PhotoActionState {
  fn new(blob: &SendWrapper<Blob>) -> Self {
    let blob_size = blob.size();
    if blob_size > MAX_UPLOAD_SIZE {
      return PhotoActionState::Oversized(FileSize::new(blob_size));
    };
    let action =
      Action::new_local(move |blob| upload_action_fn(SendWrapper::clone(blob)));
    action.dispatch_local(blob.clone());
    PhotoActionState::Started(action)
  }

  fn status(&self) -> Signal<PhotoUploadStatus> {
    match self {
      PhotoActionState::Started(action) => {
        let value = action.value();
        Signal::derive(move || {
          if value().is_some() {
            PhotoUploadStatus::UploadFinished
          } else {
            PhotoUploadStatus::UploadInProgress
          }
        })
      }
      PhotoActionState::Oversized(size) => {
        Signal::stored(PhotoUploadStatus::Oversized(size.clone()))
      }
    }
  }
}

#[derive(Debug, Clone)]
pub(super) enum PhotoUploadStatus {
  UploadInProgress,
  UploadFinished,
  Oversized(FileSize),
}

async fn upload_action_fn(
  blob: SendWrapper<Blob>,
) -> Result<ArtifactRecordId, String> {
  use gloo::net::http::*;

  let request = Request::post("/api/upload_artifact")
    .body(blob.take())
    .expect("failed to set blob as body of upload_artifact request");

  let response = request
    .send()
    .await
    .map_err(|e| format!("failed to send upload_artifact request: {e}"))?;

  let value: ArtifactRecordId = response.json().await.map_err(|e| {
    format!("failed to deserialize upload_artifact response: {e}")
  })?;

  Ok(value)
}
