use leptos::*;
use server_fn::codec::{MultipartData, MultipartFormData};
use web_sys::{wasm_bindgen::JsCast, FormData, HtmlFormElement, SubmitEvent};

#[island]
pub fn PhotoUpload() -> impl IntoView {
  let upload_action = create_action(|data: &FormData| {
    let data = data.clone();
    // `MultipartData` implements `From<FormData>`
    photo_upload(data.into())
  });
  let pending = upload_action.pending();
  let value = upload_action.value();
  let successful = move || matches!(value(), Some(Ok(_)));

  create_effect(move |_| {
    if successful() {
      crate::components::navigation::reload();
    }
  });

  view! {
    <div class="d-card bg-base-200 shadow max-w-sm">
      <div class="d-card-body">
        <p class="d-card-title text-2xl">"Upload Photo"</p>
        <form on:submit=move |ev: SubmitEvent| {
          ev.prevent_default();
          let target = ev.target().unwrap().unchecked_into::<HtmlFormElement>();
          let form_data = FormData::new_with_form(&target).unwrap();
          upload_action.dispatch(form_data);
        }>
          <div class="d-form-control">
            <label class="d-label cursor-pointer">
              <span class="d-label-text">Public</span>
              <input type="checkbox" checked="checked" name="public" class="d-checkbox" />
            </label>
          </div>
          <div class="d-form-control">
            <input type="file" class="d-file-input d-file-input-bordered w-full" name="photo" />
          </div>
          <div class="mt-6"></div>
          <div class="d-form-control">
            <button
              type="submit" class="d-btn d-btn-primary w-full"
              disabled={move || pending()}
            >
              { move || if pending() { view!{ <span class="d-loading d-loading-spinner" /> }.into_view() } else { view! {}.into_view() } }
              "Upload"
            </button>
          </div>
        </form>
      </div>
    </div>
  }
}

#[cfg_attr(feature = "ssr", tracing::instrument(skip(data)))]
#[server(input = MultipartFormData)]
pub async fn photo_upload(
  data: MultipartData,
) -> Result<core_types::PhotoGroupRecordId, ServerFnError> {
  // get the user, and abort if not logged in
  let user = crate::authenticated_user()
    .ok_or_else(|| ServerFnError::new("Not logged in"))?;

  // to upload a photo, we need the Bytes of the photo and whether it's public
  let mut public: Option<bool> = None;
  let mut photo: Option<bytes::Bytes> = None;

  // this only panics on the client
  let mut data = data.into_inner().unwrap();

  while let Some(field) = data.next_field().await.map_err(|e| {
    ServerFnError::new(format!("Failed to parse form data: {}", e))
  })? {
    match field.name() {
      Some("public") => {
        let value = field.text().await.map_err(|e| {
          ServerFnError::new(format!("Failed to read public field: {}", e))
        })?;
        public = Some(value == "on");
      }
      Some("photo") => {
        let bytes = field.bytes().await.map_err(|e| {
          ServerFnError::new(format!("Failed to read photo field: {}", e))
        })?;
        photo = Some(bytes);
      }
      _ => {
        // ignore other fields
      }
    }
  }

  let public =
    public.ok_or_else(|| ServerFnError::new("Missing public field"))?;
  let photo = photo.ok_or_else(|| ServerFnError::new("Missing photo field"))?;

  let photo_group =
    bl::upload_single_photo(user.id, photo, core_types::PhotoGroupUploadMeta {
      public,
    })
    .await
    .map_err(|e| {
      ServerFnError::new(format!("Failed to upload photo: {}", e))
    })?;

  Ok(photo_group.id)
}
