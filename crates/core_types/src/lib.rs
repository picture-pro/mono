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
//! We also have other features for when we need specialized traits implemented
//! on models, such as the [`AuthUser`](axum_login::AuthUser) trait.
//!
//! # Adding a New Table
//!
//! To add a new table, you need a record ID type, a table name constant, and a
//! model type. The record ID type should be a newtype around a `Ulid` and the
//! model type should be a struct with a `pub id: RecordId` field.
//!
//! The record ID needs the following:
//! - `#[derive(Clone, Debug, Deserialize, Serialize, Copy)]`
//! - `#[cfg_attr(feature = "ssr", serde(from = "crate::ssr::UlidOrThing"))]`:
//!   to allow deserializing from a surrealdb `Thing` when on the server.
//! - `impl_record_id!(UserRecordId, USER_TABLE);` in the `ssr` module, which
//!   implements the following:
//!   - `NewId`
//!   - `From<UlidOrThing>`
//!   - `IntoResource<Option<R>>`
//!
//! The model type needs `#[derive(Clone, Debug, Deserialize, Serialize)]`.

mod artifact;
mod auth;
mod photo;
#[cfg(feature = "ssr")]
pub(crate) mod ssr;

#[cfg(feature = "ssr")]
pub use surreal_id::NewId;
pub use ulid::Ulid;

#[cfg(feature = "ssr")]
pub use self::ssr::AsThing;
pub use self::{artifact::*, auth::*, photo::*};
