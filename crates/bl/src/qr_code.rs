use leptos::{server, server_fn::codec::Json, ServerFnError};

/// Generate a QR code from the given data. Returns base64 encoded PNG data.
#[cfg(feature = "ssr")]
pub fn generate_qr_code_inner(data: &str) -> color_eyre::eyre::Result<String> {
  use color_eyre::eyre::WrapErr;
  use qrcode::QrCode;

  let code =
    QrCode::new(data.as_bytes()).wrap_err("Failed to create QR code")?;
  let image = code.render::<image::Luma<u8>>().build();
  let mut png_bytes = Vec::new();
  let encoder = image::codecs::jpeg::JpegEncoder::new(&mut png_bytes);
  image
    .write_with_encoder(encoder)
    .wrap_err("Failed to write QR code to png")?;

  use base64::prelude::*;
  let data = BASE64_STANDARD.encode(&png_bytes);

  Ok(data)
}

#[server(
  input = Json,
  output = Json,
)]
#[cfg_attr(feature = "ssr", tracing::instrument)]
pub async fn generate_qr_code(data: String) -> Result<String, ServerFnError> {
  generate_qr_code_inner(&data)
    .map_err(|e| crate::handle_error(e, "generate qr code"))
}
