use leptos::prelude::*;

#[component]
pub fn Title(
  children: Children,
  /// Extra classes to add.
  #[prop(into, optional)]
  class: MaybeProp<String>,
) -> impl IntoView {
  const BASE_CLASS: &str = "text-6xl font-serif font-light tracking-tight mb-4";
  let final_class = Signal::derive(move || {
    format!("{BASE_CLASS} {}", class().unwrap_or_default())
  });

  view! {
    <p class=final_class>
      { children() }
    </p>
  }
}
