use color_eyre::eyre::{Result, WrapErr};
use qrcode::QrCode;
use tracing::instrument;

/// Generate a QR code from the given data. Returns base64 encoded PNG data.
#[instrument]
pub fn generate_qr_code(data: &str) -> Result<String> {
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
