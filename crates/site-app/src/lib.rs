mod components;
// mod feature_checks;
mod pages;

use leptos::prelude::*;
use leptos_meta::{
  provide_meta_context, HashedStylesheet, Link, MetaTags, Style, Title,
};
use leptos_router::{
  components::{Route, Router, Routes},
  path,
};

use self::{
  components::{Header, PageContainer},
  pages::HomePage,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
  provide_meta_context();

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
      <body class="bg-base-app text-base-normal">
        <App/>
      </body>
    </html>
  }
}

#[component]
pub fn App() -> impl IntoView {
  view! {
    <Title text="Welcome to Leptos"/>

    <Header/>
    <PageContainer>
      <Router>
        <Routes fallback=|| "Page not found.".into_view()>
          <Route path=path!("/") view=HomePage/>
          <Route path=path!("/component-testing/link") view=lsc::LinkMatrixTestPage/>
          <Route path=path!("/component-testing/button") view=lsc::ButtonMatrixTestPage/>
        </Routes>
      </Router>
    </PageContainer>
  }
}
