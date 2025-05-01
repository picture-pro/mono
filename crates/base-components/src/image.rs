#![allow(missing_docs)]

use const_format::formatcp;
use leptos::prelude::*;
use serde::{Deserialize, Serialize};

/// The style of the image.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ImageStyle {
  /// No style.
  None,
  /// A border.
  Border,
  /// A border that changes on hover.
  BorderHover,
}

const BORDER_ADD_CLASS: &str = "border-2 border-base-8 dark:border-basedark-8";
const BORDER_HOVER_ADD_CLASS: &str =
  "hover:border-primary-8 hover:dark:border-primarydark-8 ring-2 \
   ring-transparent hover:ring-primary-8 hover:dark:ring-primarydark-8";

#[component]
pub fn SmallImage(
  /// The image's URL.
  #[prop(into)]
  url: Signal<String>,
  /// The image's style.
  #[prop(into, default = ImageStyle::Border.into())]
  style: Signal<ImageStyle>,
  /// Extra classes for the image.
  #[prop(into, optional)]
  extra_class: MaybeProp<String>,
) -> impl IntoView {
  const BASE_CLASS: &str = "h-40 sm:h-48 max-w-80 sm:max-w-96 rounded-lg \
                            object-cover object-center transition";
  const BORDER_BASE_CLASS: &str = formatcp!("{BASE_CLASS} {BORDER_ADD_CLASS}");
  const BORDER_HOVER_BASE_CLASS: &str =
    formatcp!("{BORDER_BASE_CLASS} {BORDER_HOVER_ADD_CLASS}");

  let class = Signal::derive(move || {
    let base = match style() {
      ImageStyle::None => BASE_CLASS,
      ImageStyle::Border => BORDER_BASE_CLASS,
      ImageStyle::BorderHover => BORDER_HOVER_BASE_CLASS,
    };
    format!("{base} {}", extra_class.get().unwrap_or_default())
  });

  view! {
    <img src=url class=class />
  }
}

#[island]
pub fn SmallImageWithFallback(
  /// The image's URL.
  #[prop(into)]
  url: String,
  /// The image's fallback data.
  fallback_data: String,
  /// The image's style.
  #[prop(into)]
  style: ImageStyle,
  /// Extra classes for the image.
  #[prop(into, optional)]
  extra_class: Option<String>,
) -> impl IntoView {
  let url = Signal::stored(url);
  let fallback_data = Signal::stored(fallback_data);

  let loaded = RwSignal::new(false);

  let src = Signal::derive(move || match loaded.get() {
    true => url.get(),
    false => fallback_data.get(),
  });

  let onload_handler = move |_| {
    leptos::logging::log!("hit onload handler");
    loaded.set(true);
  };

  view! {
    <SmallImage url=src style=style extra_class=extra_class />
    <img class="hidden" src=url on:load=onload_handler />
  }
}
