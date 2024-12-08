use enum_iterator::Sequence;
use serde::{Deserialize, Serialize};

/// Named colors, inherited from Radix.
#[derive(
  Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Sequence,
)]
pub enum NamedColor {}

#[allow(dead_code)]
impl NamedColor {
  pub(crate) fn base_name(&self) -> &'static str { match *self {} }

  pub(crate) fn text_class(&self, shade: u8) -> String {
    format!(
      "text-{name}-{shade} dark:text-{name}dark-{shade}",
      name = self.base_name()
    )
  }
  pub(crate) fn text_class_a(&self, shade: u8) -> String {
    format!(
      "text-{name}a-{shade} dark:text-{name}darka-{shade}",
      name = self.base_name()
    )
  }

  pub(crate) fn bg_class(&self, shade: u8) -> String {
    format!(
      "bg-{name}-{shade} dark:bg-{name}dark-{shade}",
      name = self.base_name()
    )
  }
  pub(crate) fn bg_class_a(&self, shade: u8) -> String {
    format!(
      "bg-{name}a-{shade} dark:bg-{name}darka-{shade}",
      name = self.base_name()
    )
  }
  pub(crate) fn bg_class_hover(&self, shade: u8) -> String {
    format!(
      "hover:bg-{name}-{shade} dark:hover:bg-{name}dark-{shade}",
      name = self.base_name()
    )
  }
  pub(crate) fn bg_class_hover_a(&self, shade: u8) -> String {
    format!(
      "hover:bg-{name}a-{shade} dark:hover:bg-{name}darka-{shade}",
      name = self.base_name()
    )
  }
  pub(crate) fn bg_class_active(&self, shade: u8) -> String {
    format!(
      "active:bg-{name}-{shade} dark:active:bg-{name}dark-{shade}",
      name = self.base_name()
    )
  }
  pub(crate) fn bg_class_active_a(&self, shade: u8) -> String {
    format!(
      "active:bg-{name}a-{shade} dark:active:bg-{name}darka-{shade}",
      name = self.base_name()
    )
  }

  pub(crate) fn border_class(&self, shade: u8) -> String {
    format!(
      "border-{name}-{shade} dark:border-{name}dark-{shade}",
      name = self.base_name()
    )
  }
}
