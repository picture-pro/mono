use std::fmt;

use gloo::file::{Blob, ObjectUrl};
use models::{ImageRecordId, Ulid};
use send_wrapper::SendWrapper;

use super::super::photo::{Photo, PhotoUploadStatus};

pub struct UploadedPhoto {
  id:       Ulid,
  blob:     SendWrapper<Blob>,
  url:      SendWrapper<ObjectUrl>,
  image_id: ImageRecordId,
}

impl fmt::Debug for UploadedPhoto {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct(stringify!(Photo))
      .field("id", &self.id)
      .field("blob", &self.blob)
      .field("url", &self.url.to_string())
      .field("artifact_id", &self.image_id)
      .finish()
  }
}

impl UploadedPhoto {
  pub fn new(
    id: Ulid,
    blob: SendWrapper<Blob>,
    url: SendWrapper<ObjectUrl>,
    image_id: ImageRecordId,
  ) -> Self {
    Self {
      id,
      blob,
      url,
      image_id,
    }
  }

  pub fn id(&self) -> Ulid { self.id }
  pub fn url(&self) -> ObjectUrl { self.url.clone().take() }
  pub fn image_id(&self) -> ImageRecordId { self.image_id }

  pub fn from_photo(photo: &Photo) -> Option<Self> {
    match photo.upload_status()() {
      PhotoUploadStatus::UploadFinished => Some(UploadedPhoto::new(
        photo.id(),
        SendWrapper::new(photo.blob()),
        SendWrapper::new(photo.url()),
        photo.image_id()()
          .expect("photo upload status inconsistent; unable to find image_id"),
      )),
      _ => None,
    }
  }
}
