//! Field component and supporting types.

use enum_iterator::Sequence;
use leptos::prelude::*;
use serde::{Deserialize, Serialize};

/// The variant of an `lsc` [`Field`].
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
pub enum FieldVariant {
  /// A surface field.
  #[default]
  Surface,
}

/// The size of an `lsc` [`Field`].
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
pub enum FieldSize {
  /// A small field.
  Small,
  /// A medium field.
  #[default]
  Medium,
  /// A large field.
  Large,
}

/// A hint for an `lsc` [`Field`].
#[derive(
  Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Sequence,
)]
pub enum FieldHint {
  /// An error hint.
  Error,
  /// A warning hint.
  Warning,
  /// A success hint.
  Success,
}

struct FieldStyleProps {
  variant: FieldVariant,
  size:    FieldSize,
  hint:    Option<FieldHint>,
}

use FieldHint::*;
use FieldSize::*;
use FieldVariant::*;

impl FieldStyleProps {
  fn outline_class(&self) -> &'static str {
    match self.hint {
      Some(Error) => {
        "outline outline-2 outline-offset-[-1px] outline-danger-8 \
         dark:outline-dangerdark-8"
      }
      Some(Warning) => {
        "outline outline-2 outline-offset-[-1px] outline-warning-8 \
         dark:outline-warningdark-8"
      }
      Some(Success) => {
        "outline outline-2 outline-offset-[-1px] outline-success-8 \
         dark:outline-successdark-8"
      }
      None => {
        "outline-none outline-offset-[-1px] focus:outline-primary-8 \
         focus:dark:outline-primarydark-8"
      }
    }
  }

  fn color_class(&self) -> &'static str {
    match self.variant {
      Surface => {
        "text-base-12 dark:text-basedark-12 bg-surface dark:bg-surfacedark \
         border border-graya-7 dark:border-graydarka-7"
      }
    }
  }

  fn size_class(&self) -> &'static str {
    match self.size {
      Small => "text-xs indent-1.5 h-6 min-w-48",
      Medium => "text-sm indent-2 h-8 min-w-48",
      Large => "text-base indent-3 h-10 min-w-64",
    }
  }

  fn class(&self) -> String {
    format!(
      "w-full rounded-md pb-[0.5px] flex items-center transition \
       transition-[outline] placeholder:text-basea-10 \
       placeholder:dark:text-basedarka-10 {} {} {}",
      self.color_class(),
      self.size_class(),
      self.outline_class(),
    )
  }
}

/// A styled input field.
#[component]
pub fn Field(
  /// The variant of the field.
  #[prop(into, optional)]
  variant: Signal<FieldVariant>,
  /// The size of the field.
  #[prop(into, optional)]
  size: Signal<FieldSize>,
  /// The hint for the field.
  #[prop(into, optional)]
  hint: Signal<Option<FieldHint>>,
) -> impl IntoView {
  let props = move || FieldStyleProps {
    variant: variant.get(),
    size:    size.get(),
    hint:    hint.get(),
  };

  let class = Memo::new(move |_| props().class());

  view! {
    <input class=class />
  }
}

/// A test page for `lsc` [`Field`].
#[component]
pub fn FieldMatrixTestPage() -> impl IntoView {
  view! {
    <div class="flex flex-col gap-4">
      <For
        each={move || enum_iterator::all::<FieldSize>()}
        key={move |s| *s}
        children=move |size| view!{
          <div class="flex flex-col gap-2">
            <For
              each={move || enum_iterator::all::<FieldVariant>()}
              key={move |v| *v}
              children=move |variant| view!{
                <div class="flex flex-row w-full gap-2">
                  <For
                    each={move || Some(None).into_iter().chain(enum_iterator::all::<FieldHint>().map(Some))}
                    key={move |h| *h}
                    children=move |hint| view!{
                      <Field
                        size=size
                        variant=variant
                        hint={hint}
                        {..}
                        placeholder=format!("{:?} {:?} {:?}", size, variant, hint)
                      />
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
