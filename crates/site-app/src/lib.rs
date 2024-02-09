use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::error_template::{AppError, ErrorTemplate};

pub mod error_template;

#[component]
pub fn App() -> impl IntoView {
  // Provides context that manages stylesheets, titles, meta tags, etc.
  provide_meta_context();

  view! {
    <Stylesheet id="leptos" href="/pkg/site.css"/>

    // sets the document title
    <Title text="Welcome to Leptos"/>

    // content for this welcome page
    <Router fallback=|| {
      let mut outside_errors = Errors::default();
      outside_errors.insert_with_default_key(AppError::NotFound);
      view! { <ErrorTemplate outside_errors/> }.into_view()
    }>
      <main>
        <Routes>
          <Route path="" view=HomePage/>
        </Routes>
      </main>
    </Router>
  }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
  view! {
    <div class="p-4">
      <div class="flex flex-col gap-2 p-4 w-96 bg-slate-200 rounded-xl">
        <p class="text-lg">"Welcome to Leptos!"</p>
        <div>
          <p>"This is a simple example of a Leptos application."</p>
          <p>"Click the button to see how reactive values work."</p>
        </div>
        <ClickMeButton/>
      </div>
    </div>
  }
}

#[island]
fn ClickMeButton() -> impl IntoView {
  let (count, set_count) = create_signal(0);
  let on_click = move |_| set_count.update(|count| *count += 1);

  view! {
    <button on:click=on_click>"Click Me: " {count}</button>
  }
}
