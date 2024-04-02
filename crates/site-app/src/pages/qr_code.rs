use leptos::*;
use leptos_router::use_params_map;

use crate::pages::SmallPageWrapper;

#[component]
pub fn QrCodePage() -> impl IntoView {
  view! {
    <InnerQrCodePage wrapper_class="md:hidden" inner_class="border rounded-box" theme_override=Some("black") />
    <InnerQrCodePage wrapper_class="max-md:hidden" />
  }
}

#[component]
pub fn InnerQrCodePage(
  #[prop(default = "")] wrapper_class: &'static str,
  #[prop(default = "")] inner_class: &'static str,
  #[prop(default = None)] theme_override: Option<&'static str>,
) -> impl IntoView {
  let params = use_params_map();
  let id = params().get("id").cloned();

  // routing makes sure that the ID param exists
  let id = id.unwrap();
  let url = format!(
    "{}/photo/{id}",
    std::env::var("APP_BASE_URL").expect("APP_BASE_URL not set"),
  );

  // attempt to locate the photo group
  let photo_deck_element = match id.parse::<core_types::Ulid>() {
    Ok(ulid) => view! {
      <PhotoDeckWrapper group_id={core_types::PhotoGroupRecordId(ulid)} />
    }
    .into_view(),
    Err(_) => view! { <p>"Invalid photo group ID"</p> }.into_view(),
  };

  view! {
    <SmallPageWrapper extra_class=wrapper_class theme_override=theme_override>
      <div class={format!("d-card-body gap-4 {}", inner_class)}>
        {photo_deck_element}
        <div class="flex flex-row items-center gap-4">
          <a href="/dashboard" class="d-btn d-btn-primary w-full h-full flex-1">"Back to Dashboard"</a>
          <QrCode data=url class="rounded-box border shadow size-24 flex-1" />
          <a href={format!("/photo/{}", id)} class="d-btn w-full h-full flex-1">"Purchase Page"</a>
        </div>
      </div>
    </SmallPageWrapper>
  }
}

#[component]
pub fn QrCode(
  data: String,
  #[prop(default = "")] class: &'static str,
) -> impl IntoView {
  let qr_code =
    create_resource(move || data.clone(), bl::qr_code::generate_qr_code);

  view! {
    <Suspense fallback=|| view!{}>
      { qr_code.map(|r| {
        match r {
          Ok(qr_code) => view! {
            <img src={format!("data:image/png;base64,{}", qr_code)} alt="A QR code" class=class />
          }.into_view(),
          Err(e) => view! {
            <div>
              <p>{e.to_string()}</p>
              <p>"Failed to generate QR code"</p>
            </div>
          }.into_view(),
        }
      })}
    </Suspense>
  }
}

#[component]
pub fn PhotoDeckWrapper(
  group_id: core_types::PhotoGroupRecordId,
) -> impl IntoView {
  let photo_group =
    create_resource(move || group_id, bl::fetch::fetch_photo_group);

  view! {
    <Suspense fallback=|| view!{}>
      { photo_group.map(|r| {
        match r {
          Ok(Some(photo_group)) => view! {
            <div class="flex flex-row justify-center">
              <crate::components::photo_deck::PhotoDeck ids={photo_group.photos.clone()} size={crate::components::photo::PhotoSize::FitsWithinSquare(320)} />
            </div>
          }.into_view(),
          Ok(None) => view! {
            <div>
              <p>"Photo group not found"</p>
            </div>
          }.into_view(),
          Err(e) => view! {
            <div>
              <p>{e.to_string()}</p>
              <p>"Failed to fetch photo group"</p>
            </div>
          }.into_view(),
        }
      })}
    </Suspense>
  }
}
