//! Leptos application for PicturePro.

mod bridge_types;
mod components;
mod pages;
mod utils;

use leptos::prelude::*;
use leptos_meta::{
  provide_meta_context, HashedStylesheet, Link, MetaTags, Style, Title,
};
use leptos_router::{
  components::{Route, Router, Routes},
  path,
};

pub use self::bridge_types::*;
use self::{
  components::{Header, PageContainer},
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
        <Link rel="preload" href="/fonts/roboto/Roboto-Regular.ttf" as_="font" type_="font/ttf" crossorigin="anonymous" />

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
    <Title text="Welcome to Leptos"/>

    <Header/>
    <PageContainer>
      <Router>
        <Routes fallback=NotFoundPage>
          <Route path=path!("/") view=HomePage/>
          <Route path=path!("/sign-up") view=SignupPage/>
          <Route path=path!("/log-out") view=LogoutPage/>
          <Route path=path!("/component-testing/link") view=lsc::link::LinkMatrixTestPage/>
          <Route path=path!("/component-testing/button") view=lsc::button::ButtonMatrixTestPage/>
          <Route path=path!("/component-testing/field") view=lsc::field::FieldMatrixTestPage/>
        </Routes>
      </Router>
    </PageContainer>
  }
}
