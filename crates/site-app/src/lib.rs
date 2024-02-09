pub mod error_template;
pub mod utils;

use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::{
  error_template::{AppError, ErrorTemplate},
  utils::auth,
};

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
    <div class="flex flex-col justify-center items-center w-full min-h-screen">
      <div class="flex flex-col gap-2 p-4 bg-base-200 rounded-xl">
        <p class="text-lg font-semibold tracking-tight">"Welcome to Leptos!"</p>
        <div>
          <p>"This is a simple example of a Leptos application."</p>
          <p>"Click the button to see how reactive values work."</p>
        </div>
        // <div class="flex flex-row justify-end">
        //   <ClickMeButton/>
        // </div>
        <AuthStatus/>
      </div>
    </div>
  }
}

#[island]
fn ClickMeButton() -> impl IntoView {
  let (count, set_count) = create_signal(0);
  let on_click = move |_| set_count.update(|count| *count += 1);

  view! {
    <button
      on:click=on_click
      class="d-btn d-btn-primary"
    >"Click Me: " {count}</button>
  }
}

#[component]
fn AuthStatus() -> impl IntoView {
  let Some(auth_status) = auth() else {
    return view! { <p class="text-error">"Auth status not found"</p> }
      .into_view();
  };

  match auth_status.user {
    Some(user) => view! {
      <p class="text-success">
        {format!("You are logged in as {} ({})", user.name, user.id)}
      </p>
    }
    .into_view(),
    None => view! {
      <p class="text-error">
        "You are not logged in."
      </p>
    }
    .into_view(),
  }
}
