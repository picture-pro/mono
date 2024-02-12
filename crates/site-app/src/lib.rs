pub mod components;
pub mod error_template;
pub mod pages;
pub mod utils;

use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::{
  error_template::{AppError, ErrorTemplate},
  utils::authenticated_user,
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
      <div data-theme="corporate" class="w-full min-h-screen flex flex-col items-stretch justify-stretch">
        <Navbar/>
        <Routes>
          <Route path="" view=pages::home_page::HomePage/>
          <Route path="/login" view=pages::auth::login_page::LoginPage/>
          <Route path="/signup" view=pages::auth::signup_page::SignupPage/>
        </Routes>
      </div>
    </Router>
  }
}

#[component]
pub fn Navbar() -> impl IntoView {
  let current_user = authenticated_user();
  let home_url = if current_user.is_some() {
    "/dashboard"
  } else {
    "/"
  };

  view! {
    <div class="bg-base-200 w-full shadow-xl">
      <div class="d-navbar container mx-auto">
        <div class="flex-1">
          <a class="d-btn d-btn-ghost text-xl" href={home_url}>PicturePro</a>
        </div>
        <div class="flex-none flex flex-row items-center gap-2">
          <a class="d-btn d-btn-ghost" href="/login">Login</a>
          <a class="d-btn d-btn-ghost" href="/signup">Signup</a>
        </div>
      </div>
    </div>
  }
}
