use std::sync::Arc;

use argon2::{Argon2, PasswordHasher};
use password_hash::{rand_core::OsRng, SaltString};
use thiserror::Error;

use crate::repository::user::{UserRepository, RepoError};

#[async_trait::async_trait]
pub trait AuthService: Send + Sync + 'static {
    async fn signup(&self, email: &str, password: &str) -> Result<bool, AuthServiceError>;
}

#[derive(Debug, Error)]
pub enum AuthServiceError {
    #[error("password hashing failed")]
    HashError,

    #[error(transparent)]
    Repo(#[from] RepoError),
}

pub struct AuthServiceImpl {
    user_repository: Arc<dyn UserRepository>,
}

impl AuthServiceImpl {
    pub fn new(user_repository: Arc<dyn UserRepository>) -> Self {
        Self { user_repository }
    }
}

#[async_trait::async_trait]
impl AuthService for AuthServiceImpl {
    async fn signup(&self, email: &str, password: &str) -> Result<bool, AuthServiceError> {
        let salt = SaltString::generate(&mut OsRng);
        let hash = Argon2::default()
            .hash_password(password.as_bytes(), &salt)
            .map_err(|_| AuthServiceError::HashError)?
            .to_string();

        let created = self
            .user_repository
            .create_user(email, &hash)
            .await?;
        Ok(created)
    }
}

// Mocks for tests
pub struct MockAuthServiceSuccess;

#[async_trait::async_trait]
impl AuthService for MockAuthServiceSuccess {
    async fn signup(&self, _email: &str, _password: &str) -> Result<bool, AuthServiceError> {
        Ok(true)
    }
}

pub struct MockAuthServiceConflict;

#[async_trait::async_trait]
impl AuthService for MockAuthServiceConflict {
    async fn signup(&self, _email: &str, _password: &str) -> Result<bool, AuthServiceError> {
        Ok(false)
    }
}


