use std::fmt;

use gloo::file::{Blob, File, ObjectUrl};
use models::Ulid;
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
    let upload_state = match blob_size > MAX_UPLOAD_SIZE {
      true => {
        leptos::logging::warn!("photo {id} is too large ({blob_size})");
        PhotoUploadState::Oversized
      }
      false => PhotoUploadState::UploadQueued,
    };
    let url = ObjectUrl::from(blob.clone());

    Self {
      id,
      blob: SendWrapper::new(blob),
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

#[derive(Store, Debug)]
pub(super) enum PhotoUploadState {
  UploadQueued,
  UploadInProgress,
  UploadFinished,
  Oversized,
}
