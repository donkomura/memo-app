use std::sync::Arc;

use async_trait::async_trait;
use memo_app::repository::user::{RepoError, UserRepository, MockRepoSuccess, MockRepoConflict};
use memo_app::service::auth::{AuthService, AuthServiceImpl, AuthServiceError};

struct MockRepoError;

#[async_trait]
impl UserRepository for MockRepoError {
    async fn create_user(&self, _email: &str, _password_hash: &str) -> Result<bool, RepoError> {
        Err(RepoError::Internal)
    }
}

#[tokio::test]
async fn signup_returns_true_when_user_created() {
    let repo = Arc::new(MockRepoSuccess);
    let service = AuthServiceImpl::new(repo);

    let result = service.signup("a@example.com", "password").await;
    assert!(matches!(result, Ok(true)));
}

#[tokio::test]
async fn signup_returns_false_when_email_conflicts() {
    let repo = Arc::new(MockRepoConflict);
    let service = AuthServiceImpl::new(repo);

    let result = service.signup("a@example.com", "password").await;
    assert!(matches!(result, Ok(false)));
}

#[tokio::test]
async fn signup_returns_error_when_repository_fails() {
    let repo = Arc::new(MockRepoError);
    let service = AuthServiceImpl::new(repo);

    let result = service.signup("a@example.com", "password").await;
    assert!(matches!(result, Err(AuthServiceError::Repo(_))));
}


