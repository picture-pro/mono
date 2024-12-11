pub mod navigation {
  use leptos::logging;

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

  #[allow(dead_code)]
  pub fn reload() {
    let result = web_sys::window()
      .expect("Failed to get window")
      .location()
      .reload();
    if let Err(e) = result {
      logging::error!("failed to reload: {:?}", e);
    }
  }
}
