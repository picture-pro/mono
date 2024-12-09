//! Provides the [`AuthDomainService`], the entry point for users,
//! authentication, and authorization logic.

use core::fmt;
use std::sync::Arc;

use axum_login::{AuthUser, AuthnBackend};
use hex::{health, Hexagonal};
use miette::IntoDiagnostic;
use models::{
  EitherSlug, LaxSlug, User, UserAuthCredentials, UserCreateRequest,
};
use repos::{FetchModelByIndexError, FetchModelError, ModelRepository};

/// A dynamic [`AuthDomainService`] trait object.
#[derive(Clone)]
pub struct DynAuthDomainService(Arc<Box<dyn AuthDomainService>>);

impl std::ops::Deref for DynAuthDomainService {
  type Target = dyn AuthDomainService;
  fn deref(&self) -> &Self::Target { &self.0 }
}

impl DynAuthDomainService {
  /// Creates a new dynamic [`AuthDomainService`] from the given service.
  pub fn new(inner: Arc<Box<dyn AuthDomainService>>) -> Self { Self(inner) }
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

/// The authentication service trait.
#[async_trait::async_trait]
pub trait AuthDomainService: Hexagonal {
  /// Fetches a user by their ID.
  async fn fetch_user_by_id(
    &self,
    id: models::UserRecordId,
  ) -> Result<Option<User>, FetchModelError>;
  /// Fetches a user by their email address.
  async fn fetch_user_by_email(
    &self,
    email: models::EmailAddress,
  ) -> Result<Option<User>, FetchModelByIndexError>;

  /// Creates a new user.
  async fn user_signup(
    &self,
    req: UserCreateRequest,
  ) -> Result<User, CreateUserError>;

  /// Authenticates a user.
  async fn user_authenticate(
    &self,
    creds: UserAuthCredentials,
  ) -> Result<Option<User>, AuthenticationError>;
}

// smart pointer impl
#[async_trait::async_trait]
impl<T, I> AuthDomainService for T
where
  T: std::ops::Deref<Target = I> + Hexagonal + Sized,
  I: AuthDomainService + ?Sized,
{
  async fn fetch_user_by_id(
    &self,
    id: models::UserRecordId,
  ) -> Result<Option<User>, FetchModelError> {
    I::fetch_user_by_id(self, id).await
  }

  async fn fetch_user_by_email(
    &self,
    email: models::EmailAddress,
  ) -> Result<Option<User>, FetchModelByIndexError> {
    I::fetch_user_by_email(self, email).await
  }

  async fn user_signup(
    &self,
    req: UserCreateRequest,
  ) -> Result<User, CreateUserError> {
    I::user_signup(self, req).await
  }

  async fn user_authenticate(
    &self,
    creds: UserAuthCredentials,
  ) -> Result<Option<User>, AuthenticationError> {
    I::user_authenticate(self, creds).await
  }
}

/// The canonical implementation of the [`AuthDomainService`].
pub struct AuthDomainServiceCanonical<
  UR: ModelRepository<Model = User, ModelCreateRequest = UserCreateRequest>,
> {
  user_repo: UR,
}

impl<
    UR: Clone
      + ModelRepository<Model = User, ModelCreateRequest = UserCreateRequest>,
  > Clone for AuthDomainServiceCanonical<UR>
{
  fn clone(&self) -> Self {
    Self {
      user_repo: self.user_repo.clone(),
    }
  }
}

impl<
    UR: fmt::Debug
      + ModelRepository<Model = User, ModelCreateRequest = UserCreateRequest>,
  > fmt::Debug for AuthDomainServiceCanonical<UR>
{
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct(stringify!(AuthDomainServiceCanonical))
      .field("user_repo", &self.user_repo)
      .finish()
  }
}

#[async_trait::async_trait]
impl<
    UR: ModelRepository<Model = User, ModelCreateRequest = UserCreateRequest>,
  > AuthDomainService for AuthDomainServiceCanonical<UR>
{
  async fn fetch_user_by_id(
    &self,
    id: models::UserRecordId,
  ) -> Result<Option<User>, FetchModelError> {
    self.user_repo.fetch_model_by_id(id).await
  }

  async fn fetch_user_by_email(
    &self,
    email: models::EmailAddress,
  ) -> Result<Option<User>, FetchModelByIndexError> {
    self
      .user_repo
      .fetch_model_by_index(
        "email".to_string(),
        EitherSlug::Lax(LaxSlug::new(email.as_ref())),
      )
      .await
  }

  async fn user_signup(
    &self,
    req: UserCreateRequest,
  ) -> Result<User, CreateUserError> {
    let email = req.email.clone();
    if self.fetch_user_by_email(email.clone()).await?.is_some() {
      return Err(CreateUserError::EmailAlreadyUsed(email));
    }

    self
      .user_repo
      .create_model(req)
      .await
      .into_diagnostic()
      .map_err(CreateUserError::CreateError)
  }

  async fn user_authenticate(
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

impl<
    UR: ModelRepository<Model = User, ModelCreateRequest = UserCreateRequest>,
  > AuthDomainServiceCanonical<UR>
{
  /// Creates a new [`AuthDomainServiceCanonical`] with the given user
  /// repository.
  pub fn new(user_repo: UR) -> Self { Self { user_repo } }
}

#[async_trait::async_trait]
impl<
    UR: ModelRepository<Model = User, ModelCreateRequest = UserCreateRequest>,
  > health::HealthReporter for AuthDomainServiceCanonical<UR>
{
  fn name(&self) -> &'static str { stringify!(AuthDomainServiceCanonical) }
  async fn health_check(&self) -> health::ComponentHealth {
    health::AdditiveComponentHealth::from_futures(vec![self
      .user_repo
      .health_report()])
    .await
    .into()
  }
}

/// A public view of a [`User`].
#[derive(Debug, Clone)]
pub struct PublicUser {
  user:           User,
  last_auth_hash: Box<[u8]>,
}

impl From<User> for PublicUser {
  fn from(user: User) -> Self {
    let last_auth_hash = Box::from(user.auth_hash().to_be_bytes().as_slice());
    Self {
      user,
      last_auth_hash,
    }
  }
}

impl PublicUser {
  /// Returns the user's ID.
  pub fn id(&self) -> models::UserRecordId { self.user.id }
  /// Returns the user's name.
  pub fn name(&self) -> &models::HumanName { &self.user.name }
  /// Returns the user's email address.
  pub fn email(&self) -> &models::EmailAddress { &self.user.email }
  /// Returns the hash of the user's authentication secrets.
  pub fn auth_hash(&self) -> u64 { self.user.auth_hash() }
}

impl AuthUser for PublicUser {
  type Id = models::UserRecordId;
  fn id(&self) -> Self::Id { self.id() }

  fn session_auth_hash(&self) -> &[u8] { &self.last_auth_hash }
}

#[async_trait::async_trait]
impl AuthnBackend for DynAuthDomainService {
  type User = PublicUser;
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
    id: &<Self::User as AuthUser>::Id,
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
  use std::sync::Arc;

  use models::{EmailAddress, HumanName};
  use repos::MockModelRepository;

  use super::*;

  #[tokio::test]
  async fn test_user_signup() {
    let user_repo = MockModelRepository::<User, UserCreateRequest>::new();
    let service = AuthDomainServiceCanonical::new(Arc::new(user_repo));

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
    let user_repo = MockModelRepository::<User, UserCreateRequest>::new();
    let service = AuthDomainServiceCanonical::new(Arc::new(user_repo));

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
