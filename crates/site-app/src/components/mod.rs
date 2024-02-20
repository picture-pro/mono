pub mod form;
pub mod gallery;
pub mod navigation;
pub mod photo;
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
}
