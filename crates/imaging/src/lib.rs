//! Image processing.

use std::io::Cursor;

use bytes::Bytes;
use image::{ImageError, ImageFormat, imageops::FilterType};
use models::{ImageMetaData, ImageTinyPreview, MAX_TINY_PREVIEW_DIMENSION};
use thiserror::Error;

/// Image processor.
pub struct ImageProcessor {}

/// Errors for creating [`ImageMetadata`].
#[derive(Error, Debug)]
pub enum ImageCreateError {
  /// The image format could not be determined.
  #[error("The image format could not be determined.")]
  UnknownFormat,
  /// The image could not be decoded.
  #[error("The image could not be decoded: {0}")]
  DecodingFailed(ImageError),
  /// The image preview could not be encoded.
  #[error("The image preview could not be encoded: {0}")]
  PreviewEncodingFailed(ImageError),
}

impl ImageProcessor {
  /// Creates [`ImageMetadata`] from input bytes.
  pub fn image_from_bytes(
    data: Bytes,
  ) -> Result<ImageMetaData, ImageCreateError> {
    // open an image reader
    let mut reader = image::ImageReader::new(Cursor::new(data))
      .with_guessed_format()
      .expect("bytes io never fails");

    // determine format
    let format = reader.format().ok_or(ImageCreateError::UnknownFormat)?;
    reader.set_format(format);

    // decode image
    let img = reader.decode().map_err(ImageCreateError::DecodingFailed)?;

    // generate preview
    let preview = img.resize(
      MAX_TINY_PREVIEW_DIMENSION,
      MAX_TINY_PREVIEW_DIMENSION,
      FilterType::CatmullRom,
    );

    // encode preview
    let mut preview_bytes = Vec::<u8>::new();
    preview
      .write_to(&mut Cursor::new(&mut preview_bytes), ImageFormat::Avif)
      .map_err(ImageCreateError::PreviewEncodingFailed)?;

    Ok(ImageMetaData {
      width:        img.width(),
      height:       img.height(),
      tiny_preview: ImageTinyPreview {
        width:  preview.width(),
        height: preview.height(),
        data:   preview_bytes,
      },
    })
  }
}
