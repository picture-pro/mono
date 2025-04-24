use serde::{Deserialize, Serialize};

/// The base url for the whole app.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct BaseUrl(pub String);
