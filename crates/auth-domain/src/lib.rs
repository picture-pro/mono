//! Provides the [`AuthDomainService`], the entry point for users,
//! authentication, and authorization logic.

use axum_login::AuthUser as AxumLoginAuthUser;
pub use axum_login::AuthnBackend;
use hex::health::{self, HealthAware};
use miette::IntoDiagnostic;
use models::{AuthUser, User, UserAuthCredentials, UserCreateRequest};
use repos::{FetchModelByIndexError, FetchModelError, UserRepository};
use tracing::instrument;

/// The authentication session type.
pub type AuthSession = axum_login::AuthSession<AuthDomainService>;

/// A dynamic [`AuthDomainService`] trait object.
#[derive(Clone, Debug)]
pub struct AuthDomainService {
  user_repo: UserRepository,
}

impl AuthDomainService {
  /// Creates a new [`AuthDomainService`].
  #[must_use]
  pub fn new(user_repo: UserRepository) -> Self { Self { user_repo } }
}

/// An error that occurs during user creation.
#[derive(Debug, thiserror::Error, miette::Diagnostic)]
pub enum CreateUserError {
  /// Indicates that the user's email address is already in use.
  #[error("The email address is already in use: \"{0}\"")]
  EmailAlreadyUsed(models::EmailAddress),
  /// Indicates that an error occurred while creating the user.
  #[error("Failed to create the user")]
  CreateError(miette::Report),
  /// Indicates that an error occurred while fetching users by index.
  #[error("Failed to fetch users by index")]
  FetchByIndexError(#[from] FetchModelByIndexError),
}

/// An error that occurs during user authentication.
#[derive(Debug, thiserror::Error, miette::Diagnostic)]
pub enum AuthenticationError {
  /// Indicates that an error occurred while fetching users.
  #[error("Failed to fetch user")]
  FetchError(#[from] FetchModelError),
  /// Indicates that an error occurred while fetching users by index.
  #[error("Failed to fetch user by index")]
  FetchByIndexError(#[from] FetchModelByIndexError),
}

impl AuthDomainService {
  /// Fetch a [`User`] by ID.
  #[instrument(skip(self))]
  pub async fn fetch_user_by_id(
    &self,
    id: models::UserRecordId,
  ) -> Result<Option<User>, FetchModelError> {
    self.user_repo.fetch_user_by_id(id).await
  }

  /// Fetch a [`User`] by [`EmailAddress`](models::EmailAddress).
  #[instrument(skip(self))]
  pub async fn fetch_user_by_email(
    &self,
    email: models::EmailAddress,
  ) -> Result<Option<User>, FetchModelByIndexError> {
    self.user_repo.fetch_user_by_email(email).await
  }

  /// Sign up a [`User`].
  #[instrument(skip(self))]
  pub async fn user_signup(
    &self,
    req: UserCreateRequest,
  ) -> Result<User, CreateUserError> {
    let email = req.email.clone();
    if self.fetch_user_by_email(email.clone()).await?.is_some() {
      return Err(CreateUserError::EmailAlreadyUsed(email));
    }

    self
      .user_repo
      .create_user(req)
      .await
      .into_diagnostic()
      .map_err(CreateUserError::CreateError)
  }

  /// Authenticate a [`User`].
  #[instrument(skip(self))]
  pub async fn user_authenticate(
    &self,
    creds: UserAuthCredentials,
  ) -> Result<Option<User>, AuthenticationError> {
    let user = match creds {
      UserAuthCredentials::EmailEntryOnly(email) => {
        self.fetch_user_by_email(email).await?
      }
    };
    Ok(user)
  }
}

#[async_trait::async_trait]
impl health::HealthReporter for AuthDomainService {
  fn name(&self) -> &'static str { stringify!(AuthDomainService) }
  async fn health_check(&self) -> health::ComponentHealth {
    health::AdditiveComponentHealth::from_futures(vec![self
      .user_repo
      .health_report()])
    .await
    .into()
  }
}

#[async_trait::async_trait]
impl AuthnBackend for AuthDomainService {
  type User = AuthUser;
  type Credentials = UserAuthCredentials;
  type Error = AuthenticationError;

  async fn authenticate(
    &self,
    creds: Self::Credentials,
  ) -> Result<Option<Self::User>, Self::Error> {
    self
      .user_authenticate(creds)
      .await
      .map(|u| u.map(Into::into))
  }
  async fn get_user(
    &self,
    id: &<Self::User as AxumLoginAuthUser>::Id,
  ) -> Result<Option<Self::User>, Self::Error> {
    self
      .fetch_user_by_id(*id)
      .await
      .map(|u| u.map(Into::into))
      .map_err(Into::into)
  }
}

#[cfg(test)]
mod tests {
  use models::{EmailAddress, HumanName};
  use repos::db::Database;

  use super::*;

  #[tokio::test]
  async fn test_user_signup() {
    let user_repo = UserRepository::new(Database::new_mock());
    let service = AuthDomainService::new(user_repo);

    let email = EmailAddress::try_new("test@example.com").unwrap();
    let user_1_req = UserCreateRequest {
      email: email.clone(),
      name:  HumanName::try_new("Test User 1").unwrap(),
      auth:  UserAuthCredentials::EmailEntryOnly(email.clone()),
    };
    let user = service.user_signup(user_1_req).await.unwrap();
    assert_eq!(user.email, email);

    dbg!(&service);

    let user_2_req = UserCreateRequest {
      email: email.clone(),
      name:  HumanName::try_new("Test User 2").unwrap(),
      auth:  UserAuthCredentials::EmailEntryOnly(email.clone()),
    };

    let user2 = service.user_signup(user_2_req).await;
    assert!(matches!(user2, Err(CreateUserError::EmailAlreadyUsed(_))));
  }

  #[tokio::test]
  async fn test_user_authenticate() {
    let user_repo = UserRepository::new(Database::new_mock());
    let service = AuthDomainService::new(user_repo);

    let email = EmailAddress::try_new("test@example.com").unwrap();
    let user_1_req = UserCreateRequest {
      email: email.clone(),
      name:  HumanName::try_new("Test User 1").unwrap(),
      auth:  UserAuthCredentials::EmailEntryOnly(email.clone()),
    };
    let user = service.user_signup(user_1_req).await.unwrap();
    assert_eq!(user.email, email);

    let creds = UserAuthCredentials::EmailEntryOnly(email.clone());
    let auth_user = service.user_authenticate(creds).await.unwrap();
    assert_eq!(auth_user, Some(user));

    let creds = UserAuthCredentials::EmailEntryOnly(
      EmailAddress::try_new("untest@example.com").unwrap(),
    );
    let auth_user = service.user_authenticate(creds).await.unwrap();
    assert_eq!(auth_user, None);
  }
}
