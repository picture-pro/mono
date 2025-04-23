use const_format::formatcp;
use leptos::prelude::*;

/// The style of the image.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
  #[prop(into, default = ImageStyle::BorderHover.into())]
  style: Signal<ImageStyle>,
  /// Extra classes for the image.
  #[prop(into, optional)]
  extra_class: MaybeProp<String>,
) -> impl IntoView {
  const BASE_CLASS: &str = "h-40 sm:h-48 max-w-80 sm:max-w-96 rounded-lg \
                            object-cover object-center transition rounded-lg";
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
