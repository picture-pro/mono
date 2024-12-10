//! Button component and supporting types.

use enum_iterator::Sequence;
use leptos::{either::Either, prelude::*};
use serde::{Deserialize, Serialize};

use crate::colors::NamedColor;

/// The color of an `lsc` [`Button`].
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
pub enum ButtonColor {
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

/// The size of an `lsc` [`Button`].
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
pub enum ButtonSize {
  /// A small button.
  Small,
  /// A medium button.
  #[default]
  Medium,
  /// A large button.
  Large,
}

/// The variant of an `lsc` [`Button`].
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
pub enum ButtonVariant {
  /// A solid button.
  Solid,
  /// A soft button.
  #[default]
  Soft,
  /// An outlined button.
  Outline,
}

struct ButtonStyleProps {
  color:   ButtonColor,
  size:    ButtonSize,
  variant: ButtonVariant,
}

const BUTTON_SOLID_ACTIVE_FILTER: &str =
  "active:brightness-[.92] active:saturate-[1.1]";

use ButtonColor::*;
use ButtonSize::*;
use ButtonVariant::*;

impl ButtonStyleProps {
  fn text_color_class(&self) -> String {
    match self.variant {
      Solid => match self.color {
        Warning => "text-sand-12".into(),
        _ => "text-white".into(),
      },
      Soft | Outline => match self.color {
        Base => "text-basea-11 dark:text-basedarka-11".into(),
        Primary => "text-primarya-11 dark:text-primarydarka-11".into(),
        Danger => "text-dangera-11 dark:text-dangerdarka-11".into(),
        Success => "text-successa-11 dark:text-successdarka-11".into(),
        Warning => "text-warninga-11 dark:text-warningdarka-11".into(),
        Named(col) => col.text_class_a(11),
      },
    }
  }

  fn bg_color_class(&self) -> String {
    let normal = match self.variant {
      Solid => match self.color {
        Base => "bg-base-9 dark:bg-basedark-9".into(),
        Primary => "bg-primary-9 dark:bg-primarydark-9".into(),
        Danger => "bg-danger-9 dark:bg-dangerdark-9".into(),
        Success => "bg-success-9 dark:bg-successdark-9".into(),
        Warning => "bg-warning-9 dark:bg-warningdark-9".into(),
        Named(col) => col.bg_class(9),
      },
      Soft => match self.color {
        Base => "bg-basea-3 dark:bg-basedarka-3".into(),
        Primary => "bg-primarya-3 dark:bg-primarydarka-3".into(),
        Danger => "bg-dangera-3 dark:bg-dangerdarka-3".into(),
        Success => "bg-successa-3 dark:bg-successdarka-3".into(),
        Warning => "bg-warninga-3 dark:bg-warningdarka-3".into(),
        Named(col) => col.bg_class_a(3),
      },
      Outline => "bg-transparent".into(),
    };

    let hover = match self.variant {
      Solid => match self.color {
        Base => "hover:bg-base-10 dark:hover:bg-basedark-10".into(),
        Primary => "hover:bg-primary-10 dark:hover:bg-primarydark-10".into(),
        Danger => "hover:bg-danger-10 dark:hover:bg-dangerdark-10".into(),
        Success => "hover:bg-success-10 dark:hover:bg-successdark-10".into(),
        Warning => "hover:bg-warning-10 dark:hover:bg-warningdark-10".into(),
        Named(col) => col.bg_class_hover(10),
      },
      Soft => match self.color {
        Base => "hover:bg-basea-4 dark:hover:bg-basedarka-4".into(),
        Primary => "hover:bg-primarya-4 dark:hover:bg-primarydarka-4".into(),
        Danger => "hover:bg-dangera-4 dark:hover:bg-dangerdarka-4".into(),
        Success => "hover:bg-successa-4 dark:hover:bg-successdarka-4".into(),
        Warning => "hover:bg-warninga-4 dark:hover:bg-warningdarka-4".into(),
        Named(col) => col.bg_class_hover_a(4),
      },
      Outline => match self.color {
        Base => "hover:bg-basea-2 dark:hover:bg-basedarka-2".into(),
        Primary => "hover:bg-primarya-2 dark:hover:bg-primarydarka-2".into(),
        Danger => "hover:bg-dangera-2 dark:hover:bg-dangerdarka-2".into(),
        Success => "hover:bg-successa-2 dark:hover:bg-successdarka-2".into(),
        Warning => "hover:bg-warninga-2 dark:hover:bg-warningdark-2".into(),
        Named(col) => col.bg_class_hover_a(2),
      },
    };

    let active = match self.variant {
      Solid => BUTTON_SOLID_ACTIVE_FILTER.into(),
      Soft => match self.color {
        Base => "active:bg-basea-5 dark:active:bg-basedarka-5".into(),
        Primary => "active:bg-primarya-5 dark:active:bg-primarydarka-5".into(),
        Danger => "active:bg-dangera-5 dark:active:bg-dangerdarka-5".into(),
        Success => "active:bg-successa-5 dark:active:bg-successdarka-5".into(),
        Warning => "active:bg-warninga-5 dark:active:bg-warningdarka-5".into(),
        Named(col) => col.bg_class_active_a(5),
      },

      Outline => match self.color {
        Base => "active:bg-basea-3 dark:active:bg-basedarka-3".into(),
        Primary => "active:bg-primarya-3 dark:active:bg-primarydarka-3".into(),
        Danger => "active:bg-dangera-3 dark:active:bg-dangerdarka-3".into(),
        Success => "active:bg-successa-3 dark:active:bg-successdarka-3".into(),
        Warning => "active:bg-warninga-3 dark:active:bg-warningdark-3".into(),
        Named(col) => col.bg_class_active_a(3),
      },
    };

    format!("{} {} {}", normal, hover, active)
  }

  fn border_color_class(&self) -> String {
    match self.variant {
      Solid => "border-transparent".into(),
      Soft => "border-transparent".into(),
      Outline => match self.color {
        Base => "border-base-11 dark:border-basedark-11".into(),
        Primary => "border-primary-11 dark:border-primarydark-11".into(),
        Danger => "border-danger-11 dark:border-dangerdark-11".into(),
        Success => "border-success-11 dark:border-successdark-11".into(),
        Warning => "border-warning-11 dark:border-warningdark-11".into(),
        Named(col) => col.border_class(11),
      },
    }
  }

  fn color_class(&self) -> String {
    [
      self.text_color_class(),
      self.bg_color_class(),
      self.border_color_class(),
    ]
    .join(" ")
  }

  fn size_class(&self) -> String {
    match self.size {
      Small => "text-xs h-6 px-2".into(),
      Medium => "text-sm h-8 px-3".into(),
      Large => "text-base h-10 px-4".into(),
    }
  }

  fn class(&self) -> String {
    format!(
      "inline-flex items-center justify-center gap-1.5 shrink-0 transition \
       text-center rounded-md border {} {}",
      self.color_class(),
      self.size_class(),
    )
  }
}

/// A button component.
#[component]
pub fn Button(
  /// The color of the button.
  #[prop(into, optional)]
  color: Signal<ButtonColor>,
  /// The size of the button.
  #[prop(into, optional)]
  size: Signal<ButtonSize>,
  /// The variant of the button.
  #[prop(into, optional)]
  variant: Signal<ButtonVariant>,
  /// The button's HREF (optional).
  #[prop(into, optional)]
  href: Signal<Option<String>>,
  /// The button's children.
  children: Children,
) -> impl IntoView {
  let style_props = move || ButtonStyleProps {
    color:   color.get(),
    size:    size.get(),
    variant: variant.get(),
  };
  let class = Memo::new(move |_| style_props().class());

  match href.get() {
    Some(href) => Either::Left(view! {
      <a href=href class=class>
        { children() }
      </a>
    }),
    None => Either::Right(view! {
      <button class=class>
        { children() }
      </button>
    }),
  }
}

/// A test page for the `lsc` [`Button`].
#[component]
pub fn ButtonMatrixTestPage() -> impl IntoView {
  view! {
    <div class="flex flex-col gap-4">
      <For
        each={move || enum_iterator::all::<ButtonSize>()}
        key={move |s| *s}
        children=move |size| view!{
          <div class="flex flex-col gap-2">
            <For
              each={move || enum_iterator::all::<ButtonColor>()}
              key={move |c| *c}
              children=move |color| view!{
                <div class="flex flex-row gap-2">
                  <For
                    each={move || enum_iterator::all::<ButtonVariant>()}
                    key={move |v| *v}
                    children=move |variant| view!{
                      <Button color=color variant=variant size=size>
                        { format!("{:?} {:?}", color, variant) }
                      </Button>
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
