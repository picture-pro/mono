#![feature(iter_intersperse)]

pub mod auth;

pub use validator::Validate;

pub use self::auth::*;

pub trait FieldValidate {
  fn field_validate(&self, field: &str) -> Option<String>;
}

impl<T: Validate> FieldValidate for T {
  fn field_validate(&self, field: &str) -> Option<String> {
    self.validate().err().and_then(|e| {
      e.field_errors().get(field).map(|e| {
        (*e)
          .clone()
          .into_iter()
          .map(|e| e.to_string())
          .intersperse(", ".into())
          .collect()
      })
    })
  }
}
