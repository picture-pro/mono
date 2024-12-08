use enum_iterator::Sequence;
use leptos::prelude::*;
use serde::{Deserialize, Serialize};

use crate::NamedColor;

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
  #[default]
  Base,
  /// The primary color.
  Primary,
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

impl ButtonStyleProps {
  fn text_color_class(&self) -> &'static str {
    match self.variant {
      ButtonVariant::Solid => match self.color {
        ButtonColor::Base => "text-base-1 dark:text-basedark-1",
        ButtonColor::Primary => "text-primary-1 dark:text-primarydark-1",
        ButtonColor::Named(named_color) => named_color.text_class_1(),
      },
      ButtonVariant::Soft | ButtonVariant::Outline => match self.color {
        ButtonColor::Base => "text-basea-11 dark:text-basedarka-11",
        ButtonColor::Primary => "text-primarya-11 dark:text-primarydarka-11",
        ButtonColor::Named(named_color) => named_color.text_class_a11(),
      },
    }
  }

  fn bg_color_class(&self) -> String {
    match self.variant {
      ButtonVariant::Solid => match self.color {
        ButtonColor::Base => format!(
          "bg-base-9 dark:bg-basedark-9 hover:bg-base-10 \
           dark:hover:bg-basedark-10 {BUTTON_SOLID_ACTIVE_FILTER}"
        ),
        ButtonColor::Primary => {
          format!(
            "bg-primary-9 dark:bg-primarydark-9 hover:bg-primary-10 \
             dark:hover:bg-primarydark-10 {BUTTON_SOLID_ACTIVE_FILTER}"
          )
        }
        ButtonColor::Named(named_color) => format!(
          "{} {} {BUTTON_SOLID_ACTIVE_FILTER}",
          named_color.bg_class_9(),
          named_color.bg_class_hover_10(),
        ),
      },
      ButtonVariant::Soft => match self.color {
        ButtonColor::Base => "bg-basea-3 dark:bg-basedarka-3 hover:bg-basea-4 \
                              dark:hover:bg-basedarka-4 active:bg-basea-5 \
                              dark:active:bg-basedarka-5"
          .to_owned(),
        ButtonColor::Primary => {
          "bg-primarya-3 dark:bg-primarydarka-3 hover:bg-primarya-4 \
           dark:hover:bg-primarydarka-4 active:bg-primarya-5 \
           dark:active:bg-primarydarka-5"
            .to_owned()
        }
        ButtonColor::Named(named_color) => format!(
          "{} {} {}",
          named_color.bg_class_a3(),
          named_color.bg_class_hover_a4(),
          named_color.bg_class_active_a5()
        ),
      },
      ButtonVariant::Outline => match self.color {
        ButtonColor::Base => "bg-transparent dark:bg-transparent \
                              hover:bg-base-2 dark:hover:bg-basedark-2"
          .to_owned(),
        ButtonColor::Primary => "bg-transparent dark:bg-transparent \
                                 hover:bg-primary-2 \
                                 dark:hover:bg-primarydark-2"
          .to_owned(),
        ButtonColor::Named(named_color) => format!(
          "bg-transparent {} {}",
          named_color.bg_class_hover_2(),
          named_color.bg_class_active_3()
        ),
      },
    }
  }

  fn border_color_class(&self) -> &'static str {
    match self.variant {
      ButtonVariant::Solid => "border-transparent",
      ButtonVariant::Soft => "border-transparent",
      ButtonVariant::Outline => match self.color {
        ButtonColor::Base => "border-base-11 dark:border-basedark-11",
        ButtonColor::Primary => "border-primary-11 dark:border-primarydark-11",
        ButtonColor::Named(named_color) => named_color.border_class_11(),
      },
    }
  }

  fn class(&self) -> String {
    format!(
      "text-sm px-3 py-2 rounded border {} {} {}",
      self.text_color_class(),
      self.bg_color_class(),
      self.border_color_class()
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
  /// The button's children.
  children: Children,
) -> impl IntoView {
  let style_props_memo = move || ButtonStyleProps {
    color:   color.get(),
    size:    size.get(),
    variant: variant.get(),
  };
  let class = Memo::new(move |_| style_props_memo().class());

  view! {
    <button class=class>
      { children() }
    </button>
  }
}

/// A test page for the `lsc` [`Button`].
#[component]
pub fn ButtonMatrixTestPage() -> impl IntoView {
  view! {
    <div class="flex flex-row">
      <div class="grid grid-cols-3 gap-4">
        <For
          each={move || enum_iterator::all::<ButtonColor>()}
          key={move |c| *c}
          children=move |color| view!{
            <For
              each={move || enum_iterator::all::<ButtonVariant>()}
              key={move |v| *v}
              children=move |variant| view!{
                <Button color=color variant=variant>
                  { format!("{:?} {:?}", color, variant) }
                </Button>
              }
            />
          }
        />
      </div>
      <div class="flex-1" />
    </div>
  }
}
