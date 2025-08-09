use thiserror::Error;

#[async_trait::async_trait]
pub trait UserRepository: Send + Sync + 'static {
    async fn create_user(&self, email: &str, password_hash: &str) -> Result<bool, RepoError>;
}

#[derive(Debug, Error)]
pub enum RepoError {
    #[error("database error: {0}")]
    DbError(#[from] sqlx::Error),

    #[error("email already exists")]
    Conflict,

    #[error("internal error")]
    Internal,
}

// SQLite の UserRepository 実装
use sqlx::SqlitePool;

pub struct SqliteUserRepository {
    pub(crate) pool: SqlitePool,
}

impl SqliteUserRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl UserRepository for SqliteUserRepository {
    async fn create_user(&self, email: &str, password_hash: &str) -> Result<bool, RepoError> {
        let result = sqlx::query!(
            r#"INSERT INTO users (email, password_hash, created_at)
               VALUES (?, ?, strftime('%s','now'))"#,
            email,
            password_hash
        )
        .execute(&self.pool)
        .await;

        match result {
            Ok(_) => Ok(true),
            Err(e) => {
                if let sqlx::Error::Database(db_err) = &e {
                    let msg = db_err.message();
                    if msg.contains("UNIQUE constraint failed") && msg.contains("users.email") {
                        return Ok(false);
                    }
                }
                Err(RepoError::DbError(e))
            }
        }
    }
}

// Mock 実装（テストで使用）
pub struct MockRepoSuccess;
#[async_trait::async_trait]
impl UserRepository for MockRepoSuccess {
    async fn create_user(&self, _email: &str, _password_hash: &str) -> Result<bool, RepoError> {
        Ok(true)
    }
}

pub struct MockRepoConflict;
#[async_trait::async_trait]
impl UserRepository for MockRepoConflict {
    async fn create_user(&self, _email: &str, _password_hash: &str) -> Result<bool, RepoError> {
        Ok(false)
    }
}
