use leptos::prelude::*;

#[component]
pub fn ConstantHeightImage(
  /// The image's URL.
  #[prop(into)]
  url: MaybeProp<String>,
  /// Extra classes for the image.
  #[prop(into, optional)]
  extra_class: MaybeProp<String>,
) -> impl IntoView {
  const BASE_CLASS: &str = "";

  let class = move || {
    format!(
      "{BASE_CLASS} {extra_class}",
      extra_class = extra_class.get().unwrap_or_default()
    )
    .trim()
    .to_string()
  };

  view! {
    <img src=url class=class />
  }
}
