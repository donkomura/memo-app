use thiserror::Error;
use crate::domain::model::User;

pub const USERS_EMAIL_UNIQUE_CONSTRAINT: &str = "users.email"; // unique index/constraint name

#[async_trait::async_trait]
pub trait UserRepository: Send + Sync + 'static {
    async fn create_user(&self, email: &str, password_hash: &str) -> Result<Option<User>, RepoError>;
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, RepoError>;
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

// SQLite 実装はモジュールにまとめ、メッセージ文字列を定数化
pub use sqlite::SqliteUserRepository;

pub mod sqlite {
    use super::*;
    use sqlx::SqlitePool;

    pub struct SqliteUserRepository {
        pub(crate) pool: SqlitePool,
    }

    impl SqliteUserRepository {
        pub fn new(pool: SqlitePool) -> Self { Self { pool } }
    }

    #[async_trait::async_trait]
    impl UserRepository for SqliteUserRepository {
        async fn create_user(&self, email: &str, password_hash: &str) -> Result<Option<User>, RepoError> {
            let inserted = sqlx::query_as!(
                User,
                r#"INSERT INTO users (email, password_hash, created_at)
                   VALUES (?, ?, strftime('%s','now'))
                   RETURNING id, email, password_hash, created_at"#,
                email,
                password_hash
            )
            .fetch_one(&self.pool)
            .await;

            match inserted {
                Ok(user) => Ok(Some(user)),
                Err(e) => {
                    if let sqlx::Error::Database(db_err) = &e {
                        if db_err.is_unique_violation() && db_err.constraint() == Some(USERS_EMAIL_UNIQUE_CONSTRAINT) {
                            return Ok(None);
                        }
                    }
                    Err(RepoError::DbError(e))
                }
            }
        }

        async fn find_by_email(&self, email: &str) -> Result<Option<User>, RepoError> {
            let user = sqlx::query_as::<_, User>(
                r#"SELECT id, email, password_hash, created_at FROM users WHERE email = ?"#,
            )
            .bind(email)
            .fetch_optional(&self.pool)
            .await
            .map_err(RepoError::DbError)?;

            Ok(user)
        }
    }
}

// PostgreSQL の実装は feature 有効時のみモジュールにまとめる
#[cfg(feature = "postgres")]
pub use postgres::PgUserRepository;

#[cfg(feature = "postgres")]
pub mod postgres {
    use super::*;
    use sqlx::PgPool;

    pub struct PgUserRepository {
        pub(crate) pool: PgPool,
    }

    impl PgUserRepository {
        pub fn new(pool: PgPool) -> Self { Self { pool } }
    }

    #[async_trait::async_trait]
    impl UserRepository for PgUserRepository {
        async fn create_user(&self, email: &str, password_hash: &str) -> Result<Option<User>, RepoError> {
            let inserted = sqlx::query_as!(
                User,
                r#"INSERT INTO users (email, password_hash, created_at)
                   VALUES ($1, $2, EXTRACT(EPOCH FROM NOW())::bigint)
                   RETURNING id, email, password_hash, created_at"#,
                email,
                password_hash
            )
            .fetch_one(&self.pool)
            .await;

            match inserted {
                Ok(user) => Ok(Some(user)),
                Err(e) => {
                    if let sqlx::Error::Database(db_err) = &e {
                        if db_err.is_unique_violation()
                            && db_err.constraint() == Some(USERS_EMAIL_UNIQUE_CONSTRAINT)
                        {
                            return Ok(None);
                        }
                    }
                    Err(RepoError::DbError(e))
                }
            }
        }

        async fn find_by_email(&self, email: &str) -> Result<Option<User>, RepoError> {
            let user = sqlx::query_as!(
                User,
                r#"SELECT id, email, password_hash, created_at FROM users WHERE email = $1"#,
                email
            )
            .fetch_optional(&self.pool)
            .await
            .map_err(RepoError::DbError)?;
            Ok(user)
        }
    }
}

// Mock 実装（テストで使用）
pub struct MockRepoSuccess;
#[async_trait::async_trait]
impl UserRepository for MockRepoSuccess {
    async fn create_user(&self, email: &str, password_hash: &str) -> Result<Option<User>, RepoError> {
        Ok(Some(User { id: 1, email: email.to_string(), password_hash: password_hash.to_string(), created_at: 0 }))
    }

    async fn find_by_email(&self, _email: &str) -> Result<Option<User>, RepoError> {
        Ok(Some(User { id: 1, email: _email.to_string(), password_hash: "x".into(), created_at: 0 }))
    }
}

pub struct MockRepoConflict;
#[async_trait::async_trait]
impl UserRepository for MockRepoConflict {
    async fn create_user(&self, _email: &str, _password_hash: &str) -> Result<Option<User>, RepoError> {
        Ok(None)
    }

    async fn find_by_email(&self, _email: &str) -> Result<Option<User>, RepoError> {
        Ok(None)
    }
}

// モック（任意の1ユーザーを保持して検索に応答）
pub struct MockRepoWithUser {
    pub user: User,
}

#[async_trait::async_trait]
impl UserRepository for MockRepoWithUser {
    async fn create_user(&self, _email: &str, _password_hash: &str) -> Result<Option<User>, RepoError> {
        Ok(Some(self.user.clone()))
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>, RepoError> {
        if self.user.email == email {
            Ok(Some(self.user.clone()))
        } else {
            Ok(None)
        }
    }
}
