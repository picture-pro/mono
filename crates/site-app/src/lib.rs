pub mod error_template;
pub mod pages;
pub mod utils;

use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::error_template::{AppError, ErrorTemplate};

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
          <Route path="" view=pages::home_page::HomePage/>
          <Route path="/login" view=pages::auth::login_page::LoginPage/>
          <Route path="/signup" view=pages::auth::signup_page::SignupPage/>
        </Routes>
      </main>
    </Router>
  }
}
