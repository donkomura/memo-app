use std::sync::Arc;

use async_trait::async_trait;
use memo_app::repository::user::{RepoError, UserRepository, MockRepoSuccess, MockRepoConflict, MockRepoWithUser};
use memo_app::domain::model::User;
use memo_app::service::auth::{AuthService, AuthServiceImpl, AuthServiceError};

struct MockRepoError;

#[async_trait]
impl UserRepository for MockRepoError {
    async fn create_user(&self, _email: &str, _password_hash: &str) -> Result<Option<User>, RepoError> {
        Err(RepoError::Internal)
    }
    async fn find_by_email(&self, _email: &str) -> Result<Option<User>, RepoError> {
        Err(RepoError::Internal)
    }
}

#[tokio::test]
async fn signup_returns_true_when_user_created() {
    let repo = Arc::new(MockRepoSuccess);
    let service = AuthServiceImpl::new(repo);

    let result = service.signup("a@example.com", "password123").await;
    assert!(matches!(result, Ok(Some(_))));
}

#[tokio::test]
async fn signup_returns_false_when_email_conflicts() {
    let repo = Arc::new(MockRepoConflict);
    let service = AuthServiceImpl::new(repo);

    let result = service.signup("a@example.com", "password123").await;
    assert!(matches!(result, Ok(None)));
}

#[tokio::test]
async fn signup_returns_error_when_repository_fails() {
    let repo = Arc::new(MockRepoError);
    let service = AuthServiceImpl::new(repo);

    let result = service.signup("a@example.com", "password123").await;
    assert!(matches!(result, Err(AuthServiceError::Repo(_))));
}

#[tokio::test]
async fn signup_returns_error_when_email_invalid() {
    let repo = Arc::new(MockRepoSuccess);
    let service = AuthServiceImpl::new(repo);

    let result = service.signup("invalid-email", "password123").await;
    assert!(matches!(result, Err(AuthServiceError::InvalidEmail)));
}

#[tokio::test]
async fn signup_returns_error_when_password_invalid() {
    let repo = Arc::new(MockRepoSuccess);
    let service = AuthServiceImpl::new(repo);

    let result = service.signup("a@example.com", "short").await;
    assert!(matches!(result, Err(AuthServiceError::InvalidPassword)));
}

// -------- login tests ---------

fn phc(password: &str) -> String {
    // テスト用の簡易ハッシュ生成（Argon2 ベース）
    use argon2::{Argon2, PasswordHasher};
    use password_hash::{rand_core::OsRng, SaltString};
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default().hash_password(password.as_bytes(), &salt).unwrap().to_string()
}

#[tokio::test]
async fn login_returns_user_when_credentials_valid() {
    let user = User { id: 1, email: "a@example.com".into(), password_hash: phc("password123"), created_at: 0 };
    let repo = Arc::new(MockRepoWithUser { user });
    let service = AuthServiceImpl::new(repo);

    let result = service.login("a@example.com", "password123").await;
    assert!(matches!(result, Ok(Some(_))));
}

#[tokio::test]
async fn login_returns_error_when_password_wrong() {
    let user = User { id: 1, email: "a@example.com".into(), password_hash: phc("password123"), created_at: 0 };
    let repo = Arc::new(MockRepoWithUser { user });
    let service = AuthServiceImpl::new(repo);

    let result = service.login("a@example.com", "wrongpass").await;
    assert!(matches!(result, Err(AuthServiceError::InvalidCredentials)));
}

#[tokio::test]
async fn login_returns_error_when_user_not_found() {
    let user = User { id: 1, email: "b@example.com".into(), password_hash: phc("password123"), created_at: 0 };
    let repo = Arc::new(MockRepoWithUser { user });
    let service = AuthServiceImpl::new(repo);

    let result = service.login("a@example.com", "password123").await;
    assert!(matches!(result, Err(AuthServiceError::InvalidCredentials)));
}


