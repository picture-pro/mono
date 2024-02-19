use leptos::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PhotoThumbnailDisplayParams {
  data: String,
  alt:  String,
  size: (u32, u32),
}

#[component]
pub fn Photo(photo_id: core_types::PhotoRecordId) -> impl IntoView {
  let photo =
    create_resource(move || (), move |_| fetch_photo_thumbnail(photo_id));

  view! {
    <div class="bg-base-300 h-32 w-32 rounded-box">
      <Suspense fallback=|| view!{ }>
        { move || match photo() {
          Some(Ok(photo)) => {
            Some(view! {
              <img
                src={format!("data:image/png;base64,{}", photo.data)}
                alt={photo.alt} width={photo.size.0} height={photo.size.1} />
            }
            .into_view())
          }
          Some(Err(e)) => {
            Some(view! {
              <p>{ format!("Failed to load photo: {e}") }</p>
            }
            .into_view())
          }
          None => None,
        } }
      </Suspense>
    </div>
  }
}

#[cfg_attr(feature = "ssr", tracing::instrument)]
#[server]
pub async fn fetch_photo_thumbnail(
  photo_id: core_types::PhotoRecordId,
) -> Result<PhotoThumbnailDisplayParams, ServerFnError> {
  use artifact::Artifact;
  use base64::prelude::*;
  use tracing::{info_span, Instrument};

  // prep the surreal client
  let surreal_client = clients::surreal::SurrealRootClient::new()
    .instrument(info_span!("create_surreal_client"))
    .await
    .map_err(|e| {
      ServerFnError::new(format!("Failed to create surreal client: {e:?}"))
    })?;
  (*surreal_client)
    .use_ns("main")
    .use_db("main")
    .await
    .map_err(|e| {
      ServerFnError::new(format!("Failed to use namespace/db: {e:?}"))
    })?;

  // select the photo
  let photo: Option<core_types::Photo> =
    (*surreal_client).select(photo_id).await.map_err(|e| {
      ServerFnError::new(format!("Failed to select photo: {e:?}"))
    })?;
  let photo = photo.ok_or_else(|| {
    ServerFnError::new(format!("Photo not found: {photo_id:?}"))
  })?;

  // select the thumbnail artifact
  let thumbnail_artifact: Option<core_types::PublicArtifact> =
    (*surreal_client)
      .select(photo.artifacts.thumbnail.artifact_id)
      .await
      .map_err(|e| {
        ServerFnError::new(format!(
          "Failed to select thumbnail artifact: {e:?}"
        ))
      })?;
  let mut thumbnail_artifact = thumbnail_artifact.ok_or_else(|| {
    ServerFnError::new("Thumbnail artifact not found".to_string())
  })?;

  // download the thumbnail artifact and get the content
  thumbnail_artifact.download().await.map_err(|e| {
    ServerFnError::new(format!("Failed to download thumbnail: {e:?}"))
  })?;
  let thumbnail_artifact_content = match thumbnail_artifact.contents {
    Some(content) => content,
    None => {
      return Err(ServerFnError::new(
        "Thumbnail artifact content not found".to_string(),
      ))
    }
  };

  let data = BASE64_STANDARD.encode(&thumbnail_artifact_content);

  Ok(PhotoThumbnailDisplayParams {
    data,
    alt: "Thumbnail".to_string(),
    size: photo.artifacts.thumbnail.size,
  })
}
