use leptos::prelude::*;

#[component]
pub fn Prose(
  children: Children,
  /// Extra classes to add.
  #[prop(into, optional)]
  class: MaybeProp<String>,
) -> impl IntoView {
  const BASE_CLASS: &str = "text-base-dim max-w-prose";
  let final_class = Signal::derive(move || {
    format!("{BASE_CLASS} {}", class().unwrap_or_default())
  });

  view! {
    <p class=final_class>
      { children() }
    </p>
  }
}
