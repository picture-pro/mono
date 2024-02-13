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
    <Style>{include_str!("../style/fonts.css")}</Style>
    <Stylesheet id="leptos" href="/pkg/site.css"/>

    // sets the document title
    <Title text="Welcome to Leptos"/>

    // content for this welcome page
    <Router fallback=|| {
      let mut outside_errors = Errors::default();
      outside_errors.insert_with_default_key(AppError::NotFound);
      view! { <ErrorTemplate outside_errors/> }.into_view()
    }>
      <div data-theme="light" class="w-full min-h-screen flex flex-col items-stretch justify-stretch">
        <Navbar/>
        <Routes>
          <Route path="" view=pages::home_page::HomePage/>
          <Route path="/dashboard" view=pages::dashboard::DashboardPage/>
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

  let user_area = match current_user {
    Some(_user) => view! {
      <a class="d-btn d-btn-neutral d-btn-sm" href="/dashboard">Dashboard</a>
      <LogoutButton class={Some("d-btn d-btn-neutral d-btn-sm".into())} />
    }
    .into_view(),
    None => view! {
      <a class="d-btn d-btn-neutral d-btn-sm" href="/login">Login</a>
      <a class="d-btn d-btn-neutral d-btn-sm" href="/signup">Signup</a>
    }
    .into_view(),
  };

  view! {
    <div class="bg-base-200 w-full">
      <div class="d-navbar md:container md:mx-auto">
        <div class="flex-1">
          <a class="d-btn d-btn-ghost text-xl d-btn-sm" href={home_url}>PicturePro</a>
        </div>
        <div class="flex-none flex flex-row items-center gap-2">
          {user_area}
        </div>
      </div>
    </div>
  }
}

#[island]
pub fn LogoutButton(class: Option<String>) -> impl IntoView {
  let logout_action = create_server_action::<Logout>();
  let logout_value = logout_action.value();

  view! {
    <button class={class} on:click=move |_| {
      logout_action.dispatch(Logout {});
    }>"Logout"</button>
    { crate::components::navigation::ClientNav::new(
      move || matches!(logout_value(), Some(Ok(_))),
      move || "/".to_string(),
    ) }
  }
}

#[server(Logout)]
pub async fn logout() -> Result<(), ServerFnError> {
  let mut auth_session = use_context::<auth::AuthSession>()
    .ok_or_else(|| ServerFnError::new("Failed to get auth session"))?;

  auth_session.logout().await.map_err(|e| {
    logging::error!("Failed to log out: {:?}", e);
    ServerFnError::new("Failed to log out")
  })?;

  Ok(())
}
