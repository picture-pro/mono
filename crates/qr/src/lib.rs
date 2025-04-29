//! Provides domain-specific methods for creating QR codes.

use fast_qr::{QRBuilder, convert::svg::SvgBuilder, qr::QRCodeError};
use models::{BaseUrl, PhotoGroupRecordId};

/// Provides domain-specific methods for creating QR codes.
#[derive(Clone, Debug)]
pub struct QrCodeGenerator(());

impl QrCodeGenerator {
  /// Create a new [`QrCodeGenerator`].
  #[expect(clippy::new_without_default)]
  pub fn new() -> Self { QrCodeGenerator(()) }

  /// Generates a QR code for a given [`PhotoGroup`](models::PhotoGroup).
  pub fn generate_photo_group_link(
    &self,
    base_url: &BaseUrl,
    id: PhotoGroupRecordId,
  ) -> Result<String, QRCodeError> {
    const PHOTO_GROUP_PATH: &str = "/photo-group/";

    let full_url =
      format!("{base_url}{PHOTO_GROUP_PATH}{id}", base_url = base_url.0);

    let qr = QRBuilder::new(full_url).build()?;

    Ok(SvgBuilder::default().to_str(&qr))
  }
}
