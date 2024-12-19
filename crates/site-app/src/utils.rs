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

pub mod inputs {
  use leptos::{ev::Event, prelude::*};

  pub(crate) fn touched_input_bindings(
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
