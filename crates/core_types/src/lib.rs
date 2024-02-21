#![warn(missing_docs)]

//! Core types for all of picturepro.
//!
//! This crate contains the model types and ID types for the entire picturepro
//! codebase.
//!
//! This crate is isomorphic and can be used in both the client and server.
//! Generally, on the client side, models and IDs will simply serialize and
//! deserialize to themselves. On the server side, we implement a number of
//! traits to allow more convenient usage with SurrealDB.
//!
//! We also have other features for when we need third party traits implemented
//! directly on models, such as the [`AuthUser`](axum_login::AuthUser) trait.
//!
//! Generally we have two traits that enforce all of our bounds: [`CoreModel`]
//! and [`CoreId`]. These traits are implemented for all of our model types and
//! ID types with a convenience macro.
//!
//! [`CoreId`] is focused on conversions and compatibility with SurrealDB. It
//! has a `Model` associated type to make sure we keep types straight.
//!
//! [`CoreModel`] only has some bounds and the associated ID type. Other crates
//! can use [`CoreModel`] to enforce bounds and also extend the trait with
//! additional methods.
//!
//! # Adding a New Table
//!
//! To add a new table, you need a record ID type, a table name constant, and a
//! model type. The record ID type should be a newtype around a `Ulid` and the
//! model type should be a struct with a `pub id: [RecordIdType]` field.
//!
//! The record ID needs the following:
//! - `#[derive(Clone, Debug, Deserialize, Serialize, Copy)]`
//! - `#[cfg_attr(feature = "ssr", serde(from = "crate::ssr::UlidOrThing"))]`:
//!   to allow deserializing from a surrealdb `Thing` when on the server.
//! - `impl_table!(RecordIdType, ModelType, TABLE_NAME);` in the `ssr` module,
//!   which implements the following:
//!   - `CoreId<Model = ModelType>` on the record ID type
//!   - `From<UlidOrThing>` on the record ID type
//!   - `IntoResource<Option<R>>` on the record ID type
//!   - `CoreModel<Id = RecordIdType>` on the model type
//!
//! The model type needs `#[derive(Clone, Debug, Deserialize, Serialize)]`.
//!
//! # Example
//!
//! ```ignore
//! use core_types::{CoreModel, CoreId};
//! use serde::{Deserialize, Serialize};
//!
//! pub const BANANA_TABLE: &str = "example";
//!
//! #[derive(Clone, Debug, Deserialize, Serialize, Copy)]
//! #[cfg_attr(feature = "ssr", serde(from = "crate::ssr::UlidOrThing"))]
//! struct BananaRecordId(ulid::Ulid);
//!
//! #[derive(Clone, Debug, Deserialize, Serialize)]
//! struct Banana {
//!   pub id: BananaRecordId,
//!   pub name: String,
//! }
//!
//! impl_table!(BananaRecordId, Banana, BANANA_TABLE);
//! ```

mod artifact;
mod auth;
mod photo;
mod price;
#[cfg(feature = "ssr")]
pub(crate) mod ssr;

use serde::{Deserialize, Serialize};
#[cfg(feature = "ssr")]
pub use ulid::Ulid;

#[cfg(feature = "ssr")]
pub use self::ssr::{CoreId, CoreModel};
pub use self::{artifact::*, auth::*, photo::*, price::*};

#[derive(Debug, Clone, Serialize, Deserialize)]
/// The metadata for any object.
pub struct ObjectMeta {
  /// The time the object was created at.
  #[serde(with = "time::serde::timestamp")]
  pub created_at: time::OffsetDateTime,
}

impl ObjectMeta {
  /// Create a new object meta with the current time.
  pub fn new() -> Self {
    Self {
      created_at: time::OffsetDateTime::now_utc(),
    }
  }
}

impl Default for ObjectMeta {
  fn default() -> Self { Self::new() }
}
