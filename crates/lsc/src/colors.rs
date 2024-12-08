use enum_iterator::Sequence;
use serde::{Deserialize, Serialize};

/// Named colors, inherited from Radix.
#[derive(
  Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Sequence,
)]
pub enum NamedColor {
  /// Green
  Green,
  /// Red
  Red,
}

impl NamedColor {
  pub(crate) fn base_name(&self) -> &'static str {
    match self {
      NamedColor::Red => "red",
      NamedColor::Green => "green",
    }
  }

  pub(crate) fn text_class_1(&self) -> &'static str {
    match self {
      NamedColor::Green => "text-green-1 dark:text-green-1",
      NamedColor::Red => "text-red-1 dark:text-red-1",
    }
  }
  pub(crate) fn text_class_a11(&self) -> &'static str {
    match self {
      NamedColor::Green => "text-greena-11 dark:text-greena-11",
      NamedColor::Red => "text-reda-11 dark:text-reda-11",
    }
  }
  pub(crate) fn bg_class_a2(&self) -> &'static str {
    match self {
      NamedColor::Green => "bg-greena-2 dark:bg-greena-2",
      NamedColor::Red => "bg-reda-2 dark:bg-reda-2",
    }
  }
  pub(crate) fn bg_class_hover_2(&self) -> &'static str {
    match self {
      NamedColor::Green => "hover:bg-green-2 dark:hover:bg-green-2",
      NamedColor::Red => "hover:bg-red-2 dark:hover:bg-red-2",
    }
  }
  pub(crate) fn bg_class_active_3(&self) -> &'static str {
    match self {
      NamedColor::Green => "active:bg-green-3 dark:active:bg-green-3",
      NamedColor::Red => "active:bg-red-3 dark:active:bg-red-3",
    }
  }
  pub(crate) fn bg_class_3(&self) -> &'static str {
    match self {
      NamedColor::Green => "bg-green-3 dark:bg-green-3",
      NamedColor::Red => "bg-red-3 dark:bg-red-3",
    }
  }
  pub(crate) fn bg_class_a3(&self) -> &'static str {
    match self {
      NamedColor::Green => "bg-greena-3 dark:bg-greena-3",
      NamedColor::Red => "bg-reda-3 dark:bg-reda-3",
    }
  }
  pub(crate) fn bg_class_hover_a4(&self) -> &'static str {
    match self {
      NamedColor::Green => "hover:bg-green-4 dark:hover:bg-green-4",
      NamedColor::Red => "hover:bg-reda-4 dark:hover:bg-reda-4",
    }
  }
  pub(crate) fn bg_class_active_5(&self) -> &'static str {
    match self {
      NamedColor::Green => "active:bg-green-5 dark:active:bg-green-5",
      NamedColor::Red => "active:bg-red-5 dark:active:bg-red-5",
    }
  }
  pub(crate) fn bg_class_active_a5(&self) -> &'static str {
    match self {
      NamedColor::Green => "active:bg-greena-5 dark:active:bg-greena-5",
      NamedColor::Red => "active:bg-reda-5 dark:active:bg-reda-5",
    }
  }
  pub(crate) fn bg_class_9(&self) -> &'static str {
    match self {
      NamedColor::Green => "bg-green-9 dark:bg-green-9",
      NamedColor::Red => "bg-red-9 dark:bg-red-9",
    }
  }
  pub(crate) fn bg_class_hover_10(&self) -> &'static str {
    match self {
      NamedColor::Green => "hover:bg-green-10 dark:hover:bg-green-10",
      NamedColor::Red => "hover:bg-red-10 dark:hover:bg-red-10",
    }
  }
  pub(crate) fn bg_class_active_11(&self) -> &'static str {
    match self {
      NamedColor::Green => "active:bg-green-11 dark:active:bg-green-11",
      NamedColor::Red => "active:bg-red-11 dark:active:bg-red-11",
    }
  }
  pub(crate) fn border_class_11(&self) -> &'static str {
    match self {
      NamedColor::Green => "border-green-11 dark:border-green-11",
      NamedColor::Red => "border-red-11 dark:border-red-11",
    }
  }
}
