//! Link component and supporting types.

use enum_iterator::Sequence;
use leptos::prelude::*;
use serde::{Deserialize, Serialize};

use crate::colors::NamedColor;

/// The size of an `lsc` [`Link`].
#[derive(
  Clone,
  Copy,
  Default,
  Debug,
  PartialEq,
  Eq,
  Hash,
  Serialize,
  Deserialize,
  Sequence,
)]
pub enum LinkSize {
  /// A small link.
  Small,
  /// A medium link.
  #[default]
  Medium,
  /// A large link.
  Large,
  /// An extra-large link.
  ExtraLarge,
}

/// The color of an `lsc` [`Link`].
#[derive(
  Clone,
  Copy,
  Default,
  Debug,
  PartialEq,
  Eq,
  Hash,
  Serialize,
  Deserialize,
  Sequence,
)]
pub enum LinkColor {
  /// The base color.
  Base,
  /// The primary color.
  #[default]
  Primary,
  /// The danger color.
  Danger,
  /// The success color.
  Success,
  /// The warning color.
  Warning,
  /// A named color.
  Named(NamedColor),
}

/// The underline of an `lsc` [`Link`].
#[derive(
  Clone,
  Copy,
  Default,
  Debug,
  PartialEq,
  Eq,
  Hash,
  Serialize,
  Deserialize,
  Sequence,
)]
pub enum LinkUnderline {
  /// No underline.
  None,
  /// Underline on hover.
  Hover,
  /// Always underline.
  #[default]
  Always,
}

struct LinkStyleProps {
  size:          LinkSize,
  color:         LinkColor,
  high_contrast: bool,
  underline:     LinkUnderline,
}

impl LinkStyleProps {
  fn size_class(&self) -> &'static str {
    match self.size {
      LinkSize::Small => "text-sm",
      LinkSize::Medium => "text-base",
      LinkSize::Large => "text-lg",
      LinkSize::ExtraLarge => "text-xl",
    }
  }

  fn hover_class(&self) -> &'static str {
    match self.underline {
      LinkUnderline::None => "no-underline",
      // if we use "hover:underline", we don't get the transition
      LinkUnderline::Hover => {
        "underline [&:not(:hover)]:decoration-transparent"
      }
      LinkUnderline::Always => "underline",
    }
  }

  fn color_class(&self) -> String {
    match (self.color, self.high_contrast) {
      (LinkColor::Base, false) => {
        "text-basea-11 dark:text-basedarka-11".to_string()
      }
      (LinkColor::Primary, false) => {
        "text-primarya dark:text-primarydarka-11".to_string()
      }
      (LinkColor::Danger, false) => {
        "text-dangera-11 dark:text-dangerdarka-11".to_string()
      }
      (LinkColor::Success, false) => {
        "text-successa-11 dark:text-successdarka-11".to_string()
      }
      (LinkColor::Warning, false) => {
        "text-warninga-11 dark:text-warningdarka-11".to_string()
      }
      (LinkColor::Named(color), false) => color.text_class_a(11),
      (LinkColor::Base, true) => {
        "text-base-12 dark:text-basedark-12".to_string()
      }
      (LinkColor::Primary, true) => {
        "text-primarya dark:text-primarydark-12".to_string()
      }
      (LinkColor::Danger, true) => {
        "text-danger-12 dark:text-dangerdark-12".to_string()
      }
      (LinkColor::Success, true) => {
        "text-success-12 dark:text-successdark-12".to_string()
      }
      (LinkColor::Warning, true) => {
        "text-warning-12 dark:text-warningdark-12".to_string()
      }
      (LinkColor::Named(color), true) => color.text_class(12),
    }
  }

  fn class(&self) -> String {
    let base_class = "transition duration-100";

    format!(
      "{base_class} {} {} {}",
      self.size_class(),
      self.hover_class(),
      self.color_class()
    )
  }
}

/// A link component.
#[component]
pub fn Link(
  /// The color of the link.
  #[prop(into, optional)]
  color: Signal<LinkColor>,
  /// Whether the link is high-contrast.
  #[prop(into, default = false.into())]
  high_contrast: Signal<bool>,
  /// The size of the link.
  #[prop(into, optional)]
  size: Signal<LinkSize>,
  /// The underline of the link.
  #[prop(into, optional)]
  underline: Signal<LinkUnderline>,
  /// The content of the link.
  children: Children,
) -> impl IntoView {
  let style_props = move || LinkStyleProps {
    size:          size.get(),
    color:         color.get(),
    high_contrast: high_contrast.get(),
    underline:     underline.get(),
  };
  let class = Memo::new(move |_| style_props().class());

  view! {
    <a class=class>
      { children() }
    </a>
  }
}

/// A test page for the `lsc` [`Link`].
#[component]
pub fn LinkMatrixTestPage() -> impl IntoView {
  view! {
    <div class="flex flex-col gap-4">
      <For
        each={move || enum_iterator::all::<LinkSize>()}
        key=move |s| *s
        children=move |size| view! {
          <div class="flex flex-col gap-2">
            <For
              each={move || enum_iterator::all::<LinkColor>()}
              key=move |c| *c
              children=move |color| view! {
                <div class="flex flex-row gap-2">
                  <For
                    each={move || enum_iterator::all::<LinkUnderline>()}
                    key=move |u| *u
                    children=move |underline| view! {
                      <Link color=color size=size underline=underline high_contrast=false {..} href="#">
                        { format!("{:?} {:?} {:?}", color, size, underline) }
                      </Link>
                      <Link color=color size=size underline=underline high_contrast=true {..} href="#">
                        { format!("{:?} {:?} {:?} (high contrast)", color, size, underline) }
                      </Link>
                    }
                  />
                </div>
              }
            />
          </div>
        }
      />
    </div>
  }
}
