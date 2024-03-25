pub mod form;
pub mod gallery;
pub mod navigation;
pub mod photo;
pub mod photo_deck;
pub mod photo_group;
pub mod photo_upload;
pub mod user;

pub mod basic {
  use leptos::*;

  #[component]
  pub fn Link(
    children: Children,
    #[prop(optional)] href: String,
    #[prop(default = false)] subtle: bool,
    #[prop(default = false)] external: bool,
  ) -> impl IntoView {
    let target = if external { Some("_blank") } else { None };
    let rel = if external {
      Some("noopener noreferrer")
    } else {
      None
    };
    let class = if !subtle {
      "underline hover:no-underline"
    } else {
      "hover:underline"
    };

    view! {
      <a href={href} target={target} rel={rel} class={class}>
        { children() }
      </a>
    }
  }

  #[component]
  pub fn TimeAgo(time: chrono::DateTime<chrono::Utc>) -> impl IntoView {
    let formatted_time = create_blocking_resource(move || time, format_timeago);

    view! {
      <Suspense fallback=|| view!{}>
        { move || match formatted_time() {
          Some(Ok(time)) => view! {
            <span>{ time }</span>
          }.into_view(),
          _ => view! {
          <span>...</span>
          }.into_view()
        }}
      </Suspense>
    }
  }

  #[server]
  async fn format_timeago(
    date: chrono::DateTime<chrono::Utc>,
  ) -> Result<String, ServerFnError> {
    let formatter = timeago::Formatter::new();
    Ok(formatter.convert_chrono(date, chrono::Utc::now()))
  }
}
