//! Utils and functions.

pub mod navigation {
  #![allow(missing_docs)]

  //! Navigation utils.

  use leptos::{logging, prelude::*};
  use leptos_router::location::Url;

  // taken from https://github.com/leptos-rs/leptos/blob/2ee4444bb44310e73e908b98ccd2b353f534da01/router/src/location/mod.rs#L87-L100
  /// Constructs the "full path" (relative to origin, starting from "/") from a
  /// [`Url`].
  pub fn url_to_full_path(url: &Url) -> String {
    let mut path = url.path().to_string();
    if !url.search().is_empty() {
      path.push('?');
      path.push_str(url.search());
    }
    if !url.hash().is_empty() {
      if !url.hash().starts_with('#') {
        path.push('#');
      }
      path.push_str(url.hash());
    }
    path
  }

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

  #[island]
  /// Redirects to a given path when client logic loads.
  pub fn UnconditionalClientRedirect(
    /// The path to redirect to.
    path: String,
  ) -> impl IntoView {
    Effect::new(move || {
      navigate_to(&path);
    });
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
