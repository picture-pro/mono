#![feature(impl_trait_in_fn_trait_return)]

//! Leptos application for PicturePro.

mod components;
mod pages;
pub mod server_fns;

use app_upload::UploadPhotoPage;
use base_components::PageContainer;
use leptos::prelude::*;
use leptos_meta::{
  provide_meta_context, HashedStylesheet, MetaTags, Style, Title,
};
use leptos_router::{
  components::{Route, Router, Routes},
  path,
};
pub use models;

use self::{
  components::{Header, PageCover},
  pages::*,
};

/// The main shell for the Leptos application.
pub fn shell(options: LeptosOptions) -> impl IntoView {
  view! {
    <!DOCTYPE html>
    <html lang="en">
      <head>
        <meta charset="utf-8"/>
        <meta name="viewport" content="width=device-width, initial-scale=1"/>
        <AutoReload options=options.clone() />
        <HydrationScripts options=options.clone() islands=true/>

        <HashedStylesheet options id="leptos"/>
        <Style>{include_str!("../style/fonts.css")}</Style>

        <MetaTags/>
      </head>
      <body class="bg-base-app text-base-normal w-full min-h-screen flex flex-col items-stretch justify-stretch">
        <App/>
      </body>
    </html>
  }
}

/// The main application component.
#[component]
pub fn App() -> impl IntoView {
  provide_meta_context();

  view! {
    <Title text="PicturePro"/>

    <Router>
      <PageCover />
      <Header />
      <PageContainer>
        <Routes fallback=NotFoundPage>
          <Route path=path!("/") view=HomePage />
          <Route path=path!("/sign-up") view=SignupPage />
          <Route path=path!("/log-in") view=LoginPage />
          <Route path=path!("/log-out") view=protect(LogoutPage) />
          <Route path=path!("/profile") view=protect(ProfilePage) />
          <Route path=path!("/upload-photo") view=protect(UploadPhotoPage) />
          <Route path=path!("/photo-group/:id") view=PhotoGroupPage />
          <Route path=path!("/component-testing/link") view=lsc::link::LinkMatrixTestPage />
          <Route path=path!("/component-testing/button") view=lsc::button::ButtonMatrixTestPage />
          <Route path=path!("/component-testing/field") view=lsc::field::FieldMatrixTestPage />
        </Routes>
      </PageContainer>
    </Router>
  }
}

fn protect<
  F: Fn() -> O + Send + Sync + Copy + 'static,
  O: IntoView + 'static,
>(
  func: F,
) -> impl Send + Clone + 'static + Fn() -> impl IntoAny {
  move || view! { <ProtectedPage> { func() } </ProtectedPage> }
}
