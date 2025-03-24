use std::{fmt, sync::Arc};

use gloo::file::{Blob, File, ObjectUrl};
use leptos::prelude::{Action, LocalStorage};
use models::{ArtifactRecordId, Ulid};
use reactive_stores::Store;
use send_wrapper::SendWrapper;

use super::MAX_UPLOAD_SIZE;

#[derive(Store)]
pub(super) struct Photo {
  id:           Ulid,
  blob:         SendWrapper<Blob>,
  url:          SendWrapper<ObjectUrl>,
  upload_state: PhotoUploadState,
}

impl Photo {
  pub(super) fn new(file: File) -> Self {
    let id = Ulid::new();

    let blob = Blob::from(file);
    let blob_size = blob.size();
    let url = ObjectUrl::from(blob.clone());
    let blob = SendWrapper::new(blob);
    let upload_state = match blob_size > MAX_UPLOAD_SIZE {
      true => {
        leptos::logging::warn!("photo {id} is too large ({blob_size})");
        PhotoUploadState::Oversized
      }
      false => PhotoUploadState::UploadQueued(blob.clone()),
    };

    Self {
      id,
      blob,
      url: SendWrapper::new(url),
      upload_state,
    }
  }
  pub(super) fn id(&self) -> Ulid { self.id }
  pub(super) fn blob(&self) -> Blob { self.blob.clone().take() }
  pub(super) fn url(&self) -> ObjectUrl { self.url.clone().take() }
  pub(super) fn oversized(&self) -> bool {
    matches!(self.upload_state, PhotoUploadState::Oversized)
  }
  pub(super) fn upload_state(&self) -> &PhotoUploadState { &self.upload_state }
}

impl fmt::Debug for Photo {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct(stringify!(Photo))
      .field("id", &self.id)
      .field("blob", &self.blob)
      .field("url", &self.url.to_string())
      .field("upload_state", &self.upload_state)
      .finish()
  }
}

pub(super) enum PhotoUploadState {
  UploadQueued(SendWrapper<Blob>),
  UploadInProgress(
    Action<SendWrapper<Blob>, Result<ArtifactRecordId, String>, LocalStorage>,
  ),
  UploadFinished(Result<ArtifactRecordId, String>),
  Oversized,
}

impl fmt::Debug for PhotoUploadState {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::UploadQueued(_) => write!(f, "UploadQueued"),
      Self::UploadInProgress(_) => f.debug_tuple("UploadInProgress").finish(),
      Self::UploadFinished(arg0) => {
        f.debug_tuple("UploadFinished").field(arg0).finish()
      }
      Self::Oversized => write!(f, "Oversized"),
    }
  }
}

impl PhotoUploadState {
  pub(super) fn advance(&mut self) {
    match &*self {
      PhotoUploadState::UploadQueued(blob) => {
        let action = Action::new_local(move |blob| {
          upload_action_fn(SendWrapper::clone(blob))
        });
        action.dispatch_local(blob.clone());
        *self = PhotoUploadState::UploadInProgress(action);
      }
      PhotoUploadState::UploadInProgress(action) => {
        if let Some(value) = action.value()() {
          *self = PhotoUploadState::UploadFinished(value);
        }
      }
      PhotoUploadState::UploadFinished(_) => {}
      PhotoUploadState::Oversized => {}
    }
  }
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
