//! Utils and functions.

pub mod navigation {
  //! Navigation utils.

  use leptos::logging;

  /// Navigate to a new page.
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

  /// Reload the current page.
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

  /// Sanitizes the `next_url` used for redirecting from the auth pages.
  pub fn sanitize_auth_next_url(next_url: Option<String>) -> String {
    match next_url {
      Some(next_url) if next_url.starts_with("/") && next_url != "/" => {
        next_url
      }
      _ => "/profile".to_string(),
    }
  }
}

pub mod inputs {
  //! Input utils.

  use leptos::{ev::Event, prelude::*};

  /// Splits out bindings for a "touched" input signal.
  pub fn touched_input_bindings(
    s: RwSignal<Option<String>>,
  ) -> (impl Fn() -> String, impl Fn(Event)) {
    (
      move || s.get().unwrap_or_default(),
      move |e| {
        s.set(Some(event_target_value(&e)));
      },
    )
  }
}
