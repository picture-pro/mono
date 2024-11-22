use leptos::prelude::*;
use leptos_meta::{provide_meta_context, HashedStylesheet, MetaTags, Title};
use leptos_router::{
  components::{Route, Router, Routes},
  StaticSegment,
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
      <body class="bg-primary-app text-primary-normal">
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

    <Router>
      <main>
        <Routes fallback=|| "Page not found.".into_view()>
          <Route path=StaticSegment("") view=HomePage/>
        </Routes>
      </main>
    </Router>
  }
}

#[island]
fn HomePage() -> impl IntoView {
  let count = RwSignal::new(0);
  let on_click = move |_| *count.write() += 1;

  view! {
    <p>"Welcome to Leptos!"</p>
    <button
      on:click=on_click
      class=""
    >"Click Me: " {count}</button>
  }
}
