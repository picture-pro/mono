use leptos::prelude::*;
use models::PhotoGroupRecordId;

#[component]
pub fn PhotoGroupQrCode(
  /// The ID of the photo group
  id: PhotoGroupRecordId,
) -> impl IntoView {
  let url = format!("/photo-group/{id}/qr");

  view! {
    <img src=url />
  }
}
