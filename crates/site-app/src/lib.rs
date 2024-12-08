mod components;
// mod feature_checks;
mod pages;

use leptos::prelude::*;
use leptos_meta::{provide_meta_context, HashedStylesheet, MetaTags, Title};
use leptos_router::{
  components::{Route, Router, Routes},
  StaticSegment,
};

use self::{
  components::{Header, PageContainer},
  pages::HomePage,
};

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
        <MetaTags/>
      </head>
      <body class="bg-base-2 dark:bg-basedark-2 text-base-normal">
        <App/>
      </body>
    </html>
  }
}

#[component]
pub fn App() -> impl IntoView {
  provide_meta_context();

  view! {
    <Title text="Welcome to Leptos"/>

    <Header/>
    <PageContainer>
      <Router>
        <Routes fallback=|| "Page not found.".into_view()>
          <Route path=StaticSegment("") view=HomePage/>
        </Routes>
      </Router>
    </PageContainer>
  }
}
