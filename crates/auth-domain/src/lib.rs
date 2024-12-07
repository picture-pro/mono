//! Provides the [`AuthDomainService`], the entry point for users,
//! authentication, and authorization logic.

use hex::{health, Hexagonal};
use models::{
  EitherSlug, LaxSlug, User, UserAuthCredentials, UserCreateRequest,
};
use repos::{
  CreateModelError, FetchModelByIndexError, FetchModelError, ModelRepository,
};

/// An error that occurs during user creation.
#[derive(Debug, thiserror::Error, miette::Diagnostic)]
pub enum CreateUserError {
  /// Indicates that the user's email address is already in use.
  #[error("The email address is already in use: \"{0}\"")]
  EmailAlreadyUsed(models::EmailAddress),
  /// Indicates that an error occurred while creating the user.
  #[error(transparent)]
  CreateError(#[from] CreateModelError),
  /// Indicates that an error occurred while fetching users.
  #[error(transparent)]
  FetchError(#[from] FetchModelByIndexError),
}

/// An error that occurs during user authentication.
pub enum AuthenticationError {}

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

/// The canonical implementation of the [`AuthDomainService`].
pub struct AuthDomainServiceCanonical<
  UR: ModelRepository<
    Model = User,
    ModelCreateRequest = UserCreateRequest,
    CreateError = CreateModelError,
  >,
> {
  user_repo: UR,
}

#[async_trait::async_trait]
impl<
    UR: ModelRepository<
      Model = User,
      ModelCreateRequest = UserCreateRequest,
      CreateError = CreateModelError,
    >,
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

    self.user_repo.create_model(req).await.map_err(|e| e.into())
  }

  async fn user_authenticate(
    &self,
    _creds: UserAuthCredentials,
  ) -> Result<Option<User>, AuthenticationError> {
    todo!()
  }
}

impl<
    UR: ModelRepository<
      Model = User,
      ModelCreateRequest = UserCreateRequest,
      CreateError = CreateModelError,
    >,
  > AuthDomainServiceCanonical<UR>
{
  /// Creates a new [`AuthDomainServiceCanonical`] with the given user
  /// repository.
  pub fn new(user_repo: UR) -> Self { Self { user_repo } }
}

#[async_trait::async_trait]
impl<
    UR: ModelRepository<
      Model = User,
      ModelCreateRequest = UserCreateRequest,
      CreateError = CreateModelError,
    >,
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
