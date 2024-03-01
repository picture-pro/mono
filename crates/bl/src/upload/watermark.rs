use image::{DynamicImage, GenericImage, GenericImageView, Rgba};

fn create_watermark(width: u32, height: u32) -> image::RgbaImage {
  // load the watermark image from bytes which is a transparent 200x200 pixels
  // image
  let watermark_bytes = include_bytes!("../../assets/watermark.png");
  let watermark = image::load_from_memory(watermark_bytes).unwrap();

  // center the watermark inside an image of the correct size
  let mut watermark_image = image::RgbaImage::new(width, height);
  let (watermark_width, watermark_height) = watermark.dimensions();
  let x = (width - watermark_width) / 2;
  let y = (height - watermark_height) / 2;
  watermark_image.copy_from(&watermark, x, y).unwrap();

  watermark_image
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
