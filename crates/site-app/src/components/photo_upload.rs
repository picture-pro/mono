use core_types::PhotoUploadParams;
use leptos::*;
use web_sys::{wasm_bindgen::JsCast, Event, HtmlInputElement};

const MIN_PRICE: f32 = 0.1;
const MAX_PRICE: f32 = 100.0;
const DEFAULT_PRICE: f32 = 1.0;

/// Rounds a number to a given scale.
fn round_with_scale(input: f32, scale: f32) -> f32 {
  (input / scale).round() * scale
}

/// Converts a price to a reasonable multiple.
#[allow(illegal_floating_point_literal_pattern)]
fn sensible_price(input: f32) -> f32 {
  match input {
    0.0..=1.0 => round_with_scale(input, 0.05),
    1.0..=5.0 => round_with_scale(input, 0.1),
    5.0..=50.0 => round_with_scale(input, 0.25),
    _ => round_with_scale(input, 1.0),
  }
}

#[island]
pub fn PhotoUpload() -> impl IntoView {
  let (logarithmic_price, set_logarithmic_price) =
    create_signal(DEFAULT_PRICE.log10());
  let price = move || sensible_price((10.0_f32).powf(logarithmic_price()));

  let (files, set_files) = create_signal(None::<web_sys::FileList>);

  let upload_action =
    create_action(move |(file_list, price): &(web_sys::FileList, f32)| {
      let file_list = file_list.clone();
      let price = *price;

      async move {
        let images = (0..file_list.length())
          .map(|i| file_list.item(i).unwrap())
          .map(move |file| async move {
            let bytes = gloo_file::futures::read_as_bytes(&file.into())
              .await
              .unwrap();
            PhotoUploadParams { original: bytes }
          })
          .collect::<Vec<_>>();

        // await all the image futures
        let images = futures::future::join_all(images).await;

        let upload_params = core_types::PhotoGroupUploadParams {
          photos: images,
          status: core_types::PhotoGroupStatus::OwnershipForSale {
            digital_price: core_types::Price(price),
          },
        };

        bl::upload::upload_photo_group(upload_params).await
      }
    });

  let pending = upload_action.pending();
  let value = upload_action.value();

  create_effect(move |_| {
    if let Some(Ok(id)) = value() {
      // redirect to the qr code page
      let url = format!("/qr/{}", id.0);
      crate::components::navigation::navigate_to(&url);
    }
  });

  view! {
    <div class="d-card bg-base-100 shadow max-w-sm">
      <div class="d-card-body gap-4">
        <p class="text-2xl font-semibold tracking-tight">"Upload Photo"</p>

        // price input
        <div class="flex flex-row gap-4 items-center">
          <label for="price">"Price"</label>
          <input
            type="range" class="d-range" id="price" name="price"
            min={MIN_PRICE.log10()} max={MAX_PRICE.log10()} step=0.01
            on:input=move |e: Event| {
              set_logarithmic_price(event_target_value(&e).parse::<f32>().unwrap());
            }
            value={DEFAULT_PRICE.log10()}
            prop:value=logarithmic_price
          />
          <p class="min-w-[4rem] text-right">{move || format!("${:.2}", price())}</p>
        </div>

        // file input
        <input
          type="file" class="d-file-input d-file-input-bordered w-full"
          name="photo" accept="image/*" capture="camera" multiple="multiple"
          required=true on:input=move |e: Event| {
            let target = e.target().unwrap().dyn_into::<HtmlInputElement>().unwrap();
            set_files(target.files());
          }
          value=files().map(|f| f.length().to_string())
        />

        // upload button
        <div class="d-form-control mt-6">
          <button
            class="d-btn d-btn-primary w-full"
            disabled=move || pending() || files().is_none()
            on:click=move |_| upload_action.dispatch((files().unwrap(), price()))
          >
            { move || if pending() { view!{ <span class="d-loading d-loading-spinner" /> }.into_view() } else { view! {}.into_view() } }
            "Upload"
          </button>
        </div>

      </div>
    </div>
  }
}
