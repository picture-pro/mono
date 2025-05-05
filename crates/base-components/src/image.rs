#![allow(missing_docs)]

use const_format::formatcp;
use leptos::{html::Img, prelude::*};
use serde::{Deserialize, Serialize};
use web_sys::Event;

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
    <img class=class />
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

  // whether to show the full version
  let loaded = RwSignal::new(false);

  // what actually goes into the image
  let src = Signal::derive(move || match loaded.get() {
    true => url.get(),
    false => fallback_data.get(),
  });

  // fires when image fully loads, or never if cached
  let onload_handler = move |e: Event| {
    let image: web_sys::HtmlImageElement = event_target(&e);
    loaded.set(image.complete());
  };

  let image_ref = NodeRef::<Img>::new();

  // sets `loaded` immediately after render if image is cached
  Effect::watch(
    move || (),
    move |_, _, _| {
      let image = image_ref.get().unwrap();
      loaded.set(image.complete());
    },
    true,
  );

  view! {
    <SmallImage style=style extra_class={extra_class} {..} srcset=src />
    <img class="hidden" srcset=url on:load=onload_handler node_ref=image_ref />
  }
}
