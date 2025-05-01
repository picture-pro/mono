use leptos::prelude::*;
use models::{
  Image, ImageRecordId, PhotoGroup, PhotoGroupRecordId, PhotoRecordId,
};

/// Fetches the thumbnail [`Image`] of a given [`Photo`].
#[server]
pub async fn fetch_thumbnail_image_for_photo(
  /// The ID of the [`Photo`](models::Photo) to fetch.
  id: PhotoRecordId,
) -> Result<Option<Image>, ServerFnError> {
  use prime_domain::PrimeDomainService;

  let pd: PrimeDomainService = expect_context();

  let photo = pd.fetch_photo(id).await.map_err(|e| {
    tracing::error!("failed to fetch photo: {e}");
    ServerFnError::new("Internal Error")
  })?;

  let Some(photo) = photo else {
    return Ok(None);
  };

  let image_id = photo.artifacts.thumbnail;

  let image = pd
    .fetch_image(image_id)
    .await
    .map_err(|e| {
      tracing::error!("failed to fetch image: {e}");
      ServerFnError::new("Internal Error")
    })?
    .ok_or_else(|| {
      tracing::warn!("image {image_id} missing (referenced by photo {id})");
      ServerFnError::new("Internal Error")
    })?;

  Ok(Some(image))
}
