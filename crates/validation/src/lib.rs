#[warn(missing_docs)]
mod auth;

use std::str::FromStr;

pub use self::auth::*;

pub trait NewType: Sized + Clone {
  // from_str where the error type is Debug
  type Inner: 'static + Clone + ToString + FromStr;
  type Error: NewTypeError;

  fn new(inner: Self::Inner) -> Result<Self, Self::Error>;
  fn into_inner(self) -> Self::Inner;
}

pub trait NewTypeError: 'static + Sized + Clone {
  fn to_string(&self) -> String;
}

impl NewTypeError for String {
  fn to_string(&self) -> String { self.clone() }
}

impl NewTypeError for std::convert::Infallible {
  fn to_string(&self) -> String { match *self {} }
}
