/// The status of user authentication.
#[derive(Debug, Clone)]
pub struct AuthStatus(pub Option<models::PublicUser>);
