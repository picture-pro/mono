//! Provides the [`AuthDomainService`], the entry point for users,
//! authentication, and authorization logic.

use axum_login::AuthUser as AxumLoginAuthUser;
pub use axum_login::AuthnBackend;
use hex::health::{self, HealthAware};
use miette::{miette, IntoDiagnostic};
use models::{
  AuthUser, EmailAddress, HumanName, User, UserAuthCredentials,
  UserCreateRequest, UserSubmittedAuthCredentials,
};
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
  EmailAlreadyUsed(EmailAddress),
  /// Indicates than an error occurred while hashing the password.
  #[error("Failed to hash password")]
  PasswordHashing(miette::Report),
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
  /// Indicates than an error occurred while hashing the password.
  #[error("Failed to hash password")]
  PasswordHashing(miette::Report),
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
    email: EmailAddress,
  ) -> Result<Option<User>, FetchModelByIndexError> {
    self.user_repo.fetch_user_by_email(email).await
  }

  /// Sign up a [`User`].
  #[instrument(skip(self))]
  pub async fn user_signup(
    &self,
    name: HumanName,
    auth: UserSubmittedAuthCredentials,
  ) -> Result<User, CreateUserError> {
    use argon2::PasswordHasher;

    let email = match auth.clone() {
      UserSubmittedAuthCredentials::EmailAndPassword { email, .. } => email,
    };

    if self.fetch_user_by_email(email.clone()).await?.is_some() {
      return Err(CreateUserError::EmailAlreadyUsed(email));
    }

    let auth: UserAuthCredentials = match auth {
      UserSubmittedAuthCredentials::EmailAndPassword { email, password } => {
        let salt = argon2::password_hash::SaltString::generate(
          &mut argon2::password_hash::rand_core::OsRng,
        );
        let argon = argon2::Argon2::default();
        let password_hash = models::PasswordHash(
          argon
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| {
              CreateUserError::PasswordHashing(miette!(
                "failed to hash password: {e}"
              ))
            })?
            .to_string(),
        );

        UserAuthCredentials::EmailAndPassword {
          email,
          password_hash,
        }
      }
    };

    let req = UserCreateRequest { name, email, auth };

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
    creds: UserSubmittedAuthCredentials,
  ) -> Result<Option<User>, AuthenticationError> {
    use argon2::PasswordVerifier;

    let Some(user) = (match creds.clone() {
      UserSubmittedAuthCredentials::EmailAndPassword { email, .. } => {
        self.user_repo.fetch_user_by_email(email.clone()).await?
      }
    }) else {
      return Ok(None);
    };

    match (creds, user.auth.clone()) {
      (
        UserSubmittedAuthCredentials::EmailAndPassword { password, .. },
        UserAuthCredentials::EmailAndPassword { password_hash, .. },
      ) => {
        let password_hash = argon2::PasswordHash::new(&password_hash.0)
          .map_err(|e| {
            AuthenticationError::PasswordHashing(miette!(
              "failed to parse password hash: {e}"
            ))
          })?;

        let argon = argon2::Argon2::default();
        let correct =
          (match argon.verify_password(password.as_bytes(), &password_hash) {
            Ok(()) => Ok(true),
            Err(argon2::password_hash::Error::Password) => Ok(false),
            Err(e) => Err(e),
          })
          .map_err(|e| {
            AuthenticationError::PasswordHashing(miette!(
              "failed to verify password against hash: {e}"
            ))
          })?;

        Ok(correct.then_some(user))
      }
    }
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
  type Credentials = UserSubmittedAuthCredentials;
  type Error = AuthenticationError;
  type User = AuthUser;

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

    let name = HumanName::try_new("Test User 1").unwrap();
    let email = EmailAddress::try_new("test@example.com").unwrap();
    let creds = UserSubmittedAuthCredentials::EmailAndPassword {
      email:    email.clone(),
      password: "hunter42".to_string(),
    };
    let user = service.user_signup(name, creds.clone()).await.unwrap();
    assert_eq!(user.email, email);

    dbg!(&service);

    let name = HumanName::try_new("Test User 2").unwrap();
    let user2 = service.user_signup(name, creds.clone()).await;
    assert!(matches!(user2, Err(CreateUserError::EmailAlreadyUsed(_))));
  }

  #[tokio::test]
  async fn test_user_authenticate() {
    let user_repo = UserRepository::new(Database::new_mock());
    let service = AuthDomainService::new(user_repo);

    let name = HumanName::try_new("Test User 1").unwrap();
    let email = EmailAddress::try_new("test@example.com").unwrap();
    let creds = UserSubmittedAuthCredentials::EmailAndPassword {
      email:    email.clone(),
      password: "hunter42".to_string(),
    };
    let user = service.user_signup(name, creds.clone()).await.unwrap();
    assert_eq!(user.email, email);

    let auth_user = service.user_authenticate(creds).await.unwrap();
    assert_eq!(auth_user, Some(user));

    let creds = UserSubmittedAuthCredentials::EmailAndPassword {
      email:    EmailAddress::try_new("untest@example.com").unwrap(),
      password: "hunter42".to_string(),
    };
    let auth_user = service.user_authenticate(creds).await.unwrap();
    assert_eq!(auth_user, None);
  }
}
