use leptos::prelude::*;
use models::{ImageRecordId, PhotoGroupConfig, PhotoGroupRecordId};

/// Create a [`PhotoGroup`](models::PhotoGroup) from a list of
/// [`Artifact`](models::Artifact)s and a [`PhotoGroupConfig`].
#[server]
pub async fn create_photo_group_from_images(
  /// The artifact IDs to use.
  image_ids: Vec<ImageRecordId>,
  /// The photo group config to use.
  config: PhotoGroupConfig,
) -> Result<PhotoGroupRecordId, ServerFnError> {
  use models::AuthStatus;
  use prime_domain::{CreatePhotoGroupFromImagesError, PrimeDomainService};

  let auth_session: AuthStatus = expect_context();
  let Some(user) = auth_session.0 else {
    return Err(ServerFnError::new("unauthenticated"));
  };

  let pd: PrimeDomainService = expect_context();

  match pd
    .create_photo_group_from_images(image_ids, config, user.id)
    .await
  {
    Ok(pg) => Ok(pg),
    Err(CreatePhotoGroupFromImagesError::MissingImage(a)) => {
      Err(ServerFnError::new(format!("missing artifact: {a}")))
    }
    Err(e) => {
      tracing::error!("failed to create photo group from artifacts: {e}");
      Err(ServerFnError::new("internal error"))
    }
  }
}
