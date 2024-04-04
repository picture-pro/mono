#![warn(missing_docs)]

//! The `bl` crate is for business logic. Any arbitrary code that needs to run
//! on the server should be in the `bl` crate. Modules are based on semantic
//! divisions between functionality.
//!
//! ## Server Functions
//! Most of the logic here exists in the form of a server function. For this
//! reason, like other crates in picturepro, this crate has `hydrate` and `ssr`
//! features. If you add a dependency, 90% it should only exist under the `ssr`
//! feature. Only if you really need to pass a type through the API boundary
//! (over HTTP), or the crate is mostly/all macros (like [`thiserror`]), should
//! you add your dependency under the `hydrate` feature as well.
//!
//! ## Errors
//! This crate provides a [`handle_error`] method that consumes an
//! [`eyre::Report`](color_eyre::eyre::Report), properly handles it, and returns
//! a [`ServerFnError`]. It's best used by enclosing the main logic in an async
//! block (in which you use the `?` sugar), immediately awaiting the async
//! block, and then using `.map_err(|e| crate::handle_error(e, "generate
//! qr-code"))`.

pub mod fetch;
#[cfg(feature = "ssr")]
pub mod model_ext;
pub mod qr_code;
pub mod rmp_sfn;
pub mod upload;

#[cfg(feature = "ssr")]
use leptos::ServerFnError;

/// Consumes a [`eyre::Report`](color_eyre::eyre::Report), properly handles it,
/// and returns a [`ServerFnError`].
///
/// The `failed_action` parameter only used for identifying the error context in
/// logs, and it's interpolated as `"Failed to {failed_action}: {error:?}"`.
#[cfg(feature = "ssr")]
pub fn handle_error(
  error: color_eyre::eyre::Report,
  failed_action: &'static str,
) -> ServerFnError {
  tracing::error!("Failed to {failed_action}: {error:?}");
  ServerFnError::new(error)
}
