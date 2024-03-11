use exif::{DateTime, Exif, In, Tag};

/// Creates a `PhotoMeta` from an optional `Exif` object.
pub fn photo_meta_from_exif(input: Option<Exif>) -> core_types::PhotoMeta {
  let mut meta = core_types::PhotoMeta::default();

  let Some(exif) = input else {
    return meta;
  };

  // extract datetime
  if let Some(field) = exif.get_field(Tag::DateTime, In::PRIMARY) {
    match field.value {
      exif::Value::Ascii(ref vec) if !vec.is_empty() => {
        if let Ok(datetime) = DateTime::from_ascii(&vec[0]) {
          meta.date_time = chrono::NaiveDate::from_ymd_opt(
            datetime.year.into(),
            datetime.month.into(),
            datetime.day.into(),
          )
          .and_then(|date| {
            chrono::NaiveTime::from_hms_opt(
              datetime.hour.into(),
              datetime.minute.into(),
              datetime.second.into(),
            )
            .map(|time| date.and_time(time))
          });
        }
      }
      _ => {}
    }
  }

  // extract orientation
  meta.orientation = orientation_from_exif(Some(&exif));

  // extract gps
  // this isn't implemented yet

  meta
}

/// Extracts the orientation from an optional `Exif` object.
pub fn orientation_from_exif(input: Option<&Exif>) -> Option<u32> {
  if let Some(exif) = input {
    if let Some(orientation) = exif.get_field(Tag::Orientation, In::PRIMARY) {
      return orientation.value.as_uint().ok().and_then(|v| v.get(0));
    }
  }
  None
}

/// Rotates the given image based on the orientation from the exif data.
pub fn rotate_image_from_exif(
  img: &mut image::DynamicImage,
  orientation: Option<&Exif>,
) {
  let Some(value) = orientation_from_exif(orientation) else {
    return;
  };
  *img = match value {
    2 => img.fliph(),
    3 => img.rotate180(),
    4 => img.flipv(),
    5 => img.rotate90().fliph(),
    6 => img.rotate90(),
    7 => img.rotate270().fliph(),
    8 => img.rotate270(),
    _ => img.clone(),
  };
}
