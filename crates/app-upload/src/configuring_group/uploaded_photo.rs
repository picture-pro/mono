use std::fmt;

use gloo::file::{Blob, ObjectUrl};
use models::{ArtifactRecordId, Ulid};
use send_wrapper::SendWrapper;

use super::super::photo::{Photo, PhotoUploadStatus};

pub struct UploadedPhoto {
  id:          Ulid,
  blob:        SendWrapper<Blob>,
  url:         SendWrapper<ObjectUrl>,
  artifact_id: ArtifactRecordId,
}

impl fmt::Debug for UploadedPhoto {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct(stringify!(Photo))
      .field("id", &self.id)
      .field("blob", &self.blob)
      .field("url", &self.url.to_string())
      .field("artifact_id", &self.artifact_id)
      .finish()
  }
}

impl UploadedPhoto {
  pub fn new(
    id: Ulid,
    blob: SendWrapper<Blob>,
    url: SendWrapper<ObjectUrl>,
    artifact_id: ArtifactRecordId,
  ) -> Self {
    Self {
      id,
      blob,
      url,
      artifact_id,
    }
  }

  pub fn id(&self) -> Ulid { self.id }
  pub fn url(&self) -> ObjectUrl { self.url.clone().take() }
  pub fn artifact_id(&self) -> ArtifactRecordId { self.artifact_id }

  pub fn from_photo(photo: &Photo) -> Option<Self> {
    match photo.upload_status()() {
      PhotoUploadStatus::UploadFinished => Some(UploadedPhoto::new(
        photo.id(),
        SendWrapper::new(photo.blob()),
        SendWrapper::new(photo.url()),
        photo.artifact_id()().expect(
          "photo upload status inconsistent; unable to find artifact_id",
        ),
      )),
      _ => None,
    }
  }
}
