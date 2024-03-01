use image::{DynamicImage, GenericImageView};

fn create_watermark(width: u32, height: u32) -> image::DynamicImage {
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
  // center the watermark
  let x = (width - w as u32) / 2;
  let y = (height - h as u32) / 2;
  let mut final_watermark = image::RgbaImage::new(width, height);
  image::imageops::overlay(
    &mut final_watermark,
    &watermark.to_rgba8(),
    x.into(),
    y.into(),
  );

  DynamicImage::ImageRgba8(final_watermark)
}

pub fn apply_watermark(target: &mut DynamicImage) {
  // create the watermark image
  let (width, height) = target.dimensions();
  let watermark = create_watermark(width, height);

  // blend the watermark onto the target image
  image::imageops::overlay(target, &watermark, 0, 0);
}
