use leptos::*;

#[island]
pub fn ClientNav(
  #[prop(into)] is_active: RwSignal<bool>,
  #[prop(into)] path: RwSignal<String>,
) -> impl IntoView {
  create_effect(move |_| {
    if is_active.get() {
      navigate_to(&path.get());
    }
  });
}

pub fn navigate_to(path: &str) {
  logging::log!("navigating to: {}", path);
  let result = web_sys::window()
    .expect("Failed to get window")
    .location()
    .set_href(path);
  if let Err(e) = result {
    logging::error!("failed to navigate: {:?}", e);
  }
}
