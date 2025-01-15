//! Radix icons.

use leptos::prelude::*;

// macro to use radix icon svgs from the `radix-icons` directory
// import the SVG file and interpolate it into the view
macro_rules! radix_icon {
  ($component_name:ident, $file_name:literal) => {
    #[component]
    /// The `$component_name` Radix icon.
    pub fn $component_name() -> impl IntoView { include_view!($file_name) }
  };
}

radix_icon!(ArrowLeftIcon, "crates/lsc/src/radix-icons/arrow-left.svg");
radix_icon!(ArrowRightIcon, "crates/lsc/src/radix-icons/arrow-right.svg");
radix_icon!(UploadIcon, "crates/lsc/src/radix-icons/upload.svg");
