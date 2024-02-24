use leptos::*;
use leptos_router::use_params_map;

use crate::{components::basic::Link, pages::SmallPageWrapper};

#[component]
pub fn QrCodePage() -> impl IntoView {
  let params = use_params_map();
  let id = params().get("id").cloned();

  // routing makes sure that the ID param exists, but we won't check if it's
  // valid
  let id = id.unwrap();
  let url = format!(
    "{}/photo/{id}",
    std::env::var("APP_BASE_URL").expect("APP_BASE_URL not set"),
  );

  view! {
    <SmallPageWrapper>
      <div class="d-card-body">
        <div class="flex flex-row gap-4 items-center">
          <p class="text-2xl font-semibold tracking-tight">"QR Code"</p>
          <div class="flex-1" />
          <Link href=url.clone()>"Purchase Page"</Link>
        </div>
        <QrCode data=url class="rounded-box border" />
      </div>
    </SmallPageWrapper>
  }
}

#[component]
pub fn QrCode(
  data: String,
  #[prop(default = "")] class: &'static str,
) -> impl IntoView {
  let qr_code = create_resource(move || data.clone(), generate_qr_code);

  view! {
    <Suspense fallback=|| view!{}>
      { qr_code.map(|r| {
        match r {
          Ok(qr_code) => view! {
            <img src={format!("data:image/png;base64,{}", qr_code)} class=class />
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

#[server]
pub async fn generate_qr_code(data: String) -> Result<String, ServerFnError> {
  bl::qr_code::generate_qr_code(&data).map_err(|e| {
    let error = e.to_string();
    logging::error!("Failed to generate QR code: {}", error);
    ServerFnError::new(error)
  })
}
