use leptos::ServerFnError;

pub mod fetch;
#[cfg(feature = "ssr")]
pub mod model_ext;
pub mod qr_code;
pub mod rmp_sfn;
pub mod upload;

#[cfg(feature = "ssr")]
pub fn handle_error(
  error: color_eyre::eyre::Report,
  failed_action: &'static str,
) -> ServerFnError {
  tracing::error!("Failed to {}: {:?}", failed_action, &error);
  ServerFnError::new(error)
}
