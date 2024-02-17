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

#[server]
pub async fn fetch_photo_thumbnail(
  photo_id: core_types::PhotoRecordId,
) -> Result<PhotoThumbnailDisplayParams, ServerFnError> {
  let surreal_client = clients::surreal::SurrealRootClient::new()
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

  let photo: Option<core_types::Photo> =
    (*surreal_client).select(photo_id).await.map_err(|e| {
      ServerFnError::new(format!("Failed to select photo: {e:?}"))
    })?;

  let photo = photo.ok_or_else(|| {
    ServerFnError::new(format!("Photo not found: {photo_id:?}"))
  })?;

  let thumbnail_artifact: Option<core_types::PublicArtifact> =
    (*surreal_client)
      .select(photo.artifacts.thumbnail.artifact_id)
      .await
      .map_err(|e| {
        ServerFnError::new(format!(
          "Failed to select thumbnail artifact: {e:?}"
        ))
      })?;

  let thumbnail_artifact = thumbnail_artifact.ok_or_else(|| {
    ServerFnError::new("Thumbnail artifact not found".to_string())
  })?;

  // fetch the artifact using the url
  let thumbnail_image = reqwest::get(thumbnail_artifact.url)
    .await
    .map_err(|e| {
      ServerFnError::new(format!("Failed to fetch thumbnail: {e:?}"))
    })?
    .bytes()
    .await
    .map_err(|e| {
      ServerFnError::new(format!("Failed to fetch thumbnail: {e:?}"))
    })?;

  let thumbnail_image =
    image::load_from_memory(&thumbnail_image).map_err(|e| {
      ServerFnError::new(format!("Failed to load thumbnail as image: {e:?}"))
    })?;
  let mut buffer = Vec::new();
  let mut encoder =
    image::codecs::jpeg::JpegEncoder::new_with_quality(&mut buffer, 95);
  encoder.encode_image(&thumbnail_image).map_err(|e| {
    ServerFnError::new(format!("Failed to encode thumbnail: {e:?}"))
  })?;

  let data = format!("data:image/jpeg;base64,{}", base64::encode(&buffer));

  Ok(PhotoThumbnailDisplayParams {
    data,
    alt: "Thumbnail".to_string(),
    size: photo.artifacts.thumbnail.size,
  })
}
