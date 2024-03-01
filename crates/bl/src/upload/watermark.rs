use image::{DynamicImage, GenericImage, GenericImageView};

fn create_watermark(width: u32, height: u32) -> image::RgbaImage {
  // load the watermark image from bytes which is a transparent 200x200 pixels
  // image
  let watermark_bytes = include_bytes!("../../assets/watermark.png");
  let watermark = image::load_from_memory(watermark_bytes).unwrap();

  // center the watermark inside an image of the correct size, or if it's too
  // large, scale it down
  let (w, h) = watermark.dimensions();
  let (w, h) = if w > width || h > height {
    let scale = (width as f32 / w as f32).min(height as f32 / h as f32);
    (w as f32 * scale, h as f32 * scale)
  } else {
    (w as f32, h as f32)
  };
  let watermark = watermark.resize_exact(
    w as u32,
    h as u32,
    image::imageops::FilterType::Lanczos3,
  );

  watermark.into()
}

pub fn apply_watermark(target: &mut DynamicImage) {
  // create the watermark image
  let (width, height) = target.dimensions();
  let watermark = create_watermark(width, height);
  let watermark = DynamicImage::ImageRgba8(watermark);

  // blend the watermark onto the target image
  for (x, y, pixel) in watermark.pixels() {
    target.put_pixel(x, y, pixel);
  }
}
