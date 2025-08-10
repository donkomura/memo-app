use std::sync::Arc;

use argon2::{Argon2, PasswordHasher};
use password_hash::{PasswordHash, PasswordVerifier, SaltString, rand_core::OsRng};
use thiserror::Error;

use crate::domain::model::User;
use crate::repository::user::{RepoError, UserRepository};

/// 認証に関するユースケースを提供するサービス層。
#[async_trait::async_trait]
pub trait AuthService: Send + Sync + 'static {
    /// ユーザー登録を行う。
    ///
    /// 入力バリデーション:
    /// - email: 簡易フォーマット検証（`local@domain.tld` 形式。ドメインに `.` を含むこと）
    /// - password: 以下のポリシーを満たす必要があります
    ///   - 8文字以上
    ///   - ASCII 英字を1文字以上含む
    ///   - ASCII 数字を1文字以上含む
    ///
    /// 返り値:
    /// - Ok(Some(User)): ユーザー作成に成功（作成された `User` を返す）
    /// - Ok(None): email が既に存在（競合）
    /// - Err(InvalidEmail | InvalidPassword): バリデーション違反
    /// - Err(Repo(_)) / Err(HashError): 内部エラー
    async fn signup(&self, email: &str, password: &str) -> Result<Option<User>, AuthServiceError>;

    async fn login(&self, email: &str, password: &str) -> Result<Option<User>, AuthServiceError>;
}

#[derive(Debug, Error)]
pub enum AuthServiceError {
    #[error("password hashing failed")]
    HashError,

    #[error(transparent)]
    Repo(#[from] RepoError),

    #[error("invalid email format")]
    InvalidEmail,

    #[error("invalid password")]
    InvalidPassword,

    #[error("invalid credentials")]
    InvalidCredentials,
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
    async fn signup(&self, email: &str, password: &str) -> Result<Option<User>, AuthServiceError> {
        if !is_valid_email(email) {
            return Err(AuthServiceError::InvalidEmail);
        }
        if !is_valid_password(password) {
            return Err(AuthServiceError::InvalidPassword);
        }
        let salt = SaltString::generate(&mut OsRng);
        let hash = Argon2::default()
            .hash_password(password.as_bytes(), &salt)
            .map_err(|_| AuthServiceError::HashError)?
            .to_string();

        let created = self.user_repository.create_user(email, &hash).await?;
        Ok(created)
    }

    async fn login(&self, email: &str, password: &str) -> Result<Option<User>, AuthServiceError> {
        let Some(user) = self.user_repository.find_by_email(email).await? else {
            return Err(AuthServiceError::InvalidCredentials);
        };

        // パスワードの検証
        let parsed = PasswordHash::new(&user.password_hash)
            .map_err(|_| AuthServiceError::InvalidCredentials)?;
        Argon2::default()
            .verify_password(password.as_bytes(), &parsed)
            .map_err(|_| AuthServiceError::InvalidCredentials)?;

        Ok(Some(user))
    }
}

// Mocks for tests
pub struct MockAuthServiceSuccess;

#[async_trait::async_trait]
impl AuthService for MockAuthServiceSuccess {
    async fn signup(&self, email: &str, _password: &str) -> Result<Option<User>, AuthServiceError> {
        Ok(Some(User {
            id: 1,
            email: email.to_string(),
            password_hash: "x".into(),
            created_at: 0,
        }))
    }

    async fn login(&self, _email: &str, _password: &str) -> Result<Option<User>, AuthServiceError> {
        Ok(Some(User {
            id: 1,
            email: _email.to_string(),
            password_hash: _password.into(),
            created_at: 0,
        }))
    }
}

pub struct MockAuthServiceConflict;

#[async_trait::async_trait]
impl AuthService for MockAuthServiceConflict {
    async fn signup(
        &self,
        _email: &str,
        _password: &str,
    ) -> Result<Option<User>, AuthServiceError> {
        Ok(None)
    }

    async fn login(&self, _email: &str, _password: &str) -> Result<Option<User>, AuthServiceError> {
        Ok(None)
    }
}

/// 簡易 email 検証。
/// - `local@domain.tld` の形式で、ドメイン部に少なくとも1つの `.` を含むこと
/// - 空白文字は不可
/// - 長さは 3..=254 文字
fn is_valid_email(email: &str) -> bool {
    if email.len() < 3 || email.len() > 254 {
        return false;
    }
    if email.contains(' ') {
        return false;
    }
    let parts: Vec<&str> = email.split('@').collect();
    if parts.len() != 2 {
        return false;
    }
    let (local, domain) = (parts[0], parts[1]);
    if local.is_empty() || domain.is_empty() {
        return false;
    }
    if !domain.contains('.') {
        return false;
    }
    true
}

/// パスワードの検証。
/// ポリシー:
/// - 8文字以上
/// - ASCII 英字を1文字以上含む
/// - ASCII 数字を1文字以上含む
fn is_valid_password(password: &str) -> bool {
    if password.len() < 8 {
        return false;
    }
    let mut has_alpha = false;
    let mut has_digit = false;
    for ch in password.chars() {
        if ch.is_ascii_alphabetic() {
            has_alpha = true;
        }
        if ch.is_ascii_digit() {
            has_digit = true;
        }
        if has_alpha && has_digit {
            return true;
        }
    }
    false
}
