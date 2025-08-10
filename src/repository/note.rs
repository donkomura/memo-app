use crate::domain::model::Note;
use crate::repository::user::RepoError;

#[async_trait::async_trait]
pub trait NoteRepository: Send + Sync + 'static {
    async fn create_note(
        &self,
        user_id: i64,
        title: &str,
        content: &str,
    ) -> Result<Note, RepoError>;
    async fn find_by_id(&self, note_id: i64) -> Result<Option<Note>, RepoError>;
    async fn update_note(
        &self,
        note_id: i64,
        user_id: i64,
        title: Option<&str>,
        content: Option<&str>,
    ) -> Result<Option<Note>, RepoError>;
    async fn delete_note(&self, note_id: i64, user_id: i64) -> Result<bool, RepoError>;
    async fn list_notes(&self) -> Result<Vec<Note>, RepoError>;
}
// SQLite 実装をモジュールにまとめる
#[cfg(not(feature = "postgres"))]
pub use sqlite::SqliteNoteRepository;

#[cfg(not(feature = "postgres"))]
pub mod sqlite {
    use super::*;
    use sqlx::SqlitePool;

    pub struct SqliteNoteRepository {
        pub(crate) pool: SqlitePool,
    }

    impl SqliteNoteRepository {
        pub fn new(pool: SqlitePool) -> Self {
            Self { pool }
        }
    }

    #[async_trait::async_trait]
    impl NoteRepository for SqliteNoteRepository {
        async fn create_note(
            &self,
            user_id: i64,
            title: &str,
            content: &str,
        ) -> Result<Note, RepoError> {
            let inserted = sqlx::query_as::<sqlx::Sqlite, Note>(
                r#"INSERT INTO notes (user_id, title, content, created_at, updated_at)
                   VALUES (?, ?, ?, strftime('%s','now'), strftime('%s','now'))
                   RETURNING id, user_id as author_id, title, content, created_at, updated_at"#,
            )
            .bind(user_id)
            .bind(title)
            .bind(content)
            .fetch_one(&self.pool)
            .await
            .map_err(RepoError::DbError)?;

            Ok(inserted)
        }
        async fn find_by_id(&self, note_id: i64) -> Result<Option<Note>, RepoError> {
            let note = sqlx::query_as::<sqlx::Sqlite, Note>(
                r#"SELECT id, user_id as author_id, title, content, created_at, updated_at
                   FROM notes
                   WHERE id = ?"#,
            )
            .bind(note_id)
            .fetch_optional(&self.pool)
            .await
            .map_err(RepoError::DbError)?;

            Ok(note)
        }

        async fn update_note(
            &self,
            note_id: i64,
            user_id: i64,
            title: Option<&str>,
            content: Option<&str>,
        ) -> Result<Option<Note>, RepoError> {
            let updated = sqlx::query_as::<sqlx::Sqlite, Note>(
                r#"UPDATE notes
                   SET title = COALESCE(?, title),
                       content = COALESCE(?, content),
                       updated_at = strftime('%s','now')
                   WHERE id = ? AND user_id = ?
                   RETURNING id, user_id as author_id, title, content, created_at, updated_at"#,
            )
            .bind(title)
            .bind(content)
            .bind(note_id)
            .bind(user_id)
            .fetch_optional(&self.pool)
            .await
            .map_err(RepoError::DbError)?;

            Ok(updated)
        }
        async fn delete_note(&self, note_id: i64, user_id: i64) -> Result<bool, RepoError> {
            let result =
                sqlx::query::<sqlx::Sqlite>(r#"DELETE FROM notes WHERE id = ? AND user_id = ?"#)
                    .bind(note_id)
                    .bind(user_id)
                    .execute(&self.pool)
                    .await
                    .map_err(RepoError::DbError)?;
            Ok(result.rows_affected() > 0)
        }
        async fn list_notes(&self) -> Result<Vec<Note>, RepoError> {
            let notes = sqlx::query_as::<sqlx::Sqlite, Note>(
                r#"SELECT id, user_id as author_id, title, content, created_at, updated_at
                   FROM notes
                   ORDER BY created_at DESC"#,
            )
            .fetch_all(&self.pool)
            .await
            .map_err(RepoError::DbError)?;
            Ok(notes)
        }
    }
}

// PostgreSQL 実装をモジュールにまとめる
#[cfg(feature = "postgres")]
pub use postgres::PgNoteRepository;

#[cfg(feature = "postgres")]
pub mod postgres {
    use super::*;
    use sqlx::PgPool;

    pub struct PgNoteRepository {
        pub(crate) pool: PgPool,
    }

    impl PgNoteRepository {
        pub fn new(pool: PgPool) -> Self {
            Self { pool }
        }
    }

    #[async_trait::async_trait]
    impl NoteRepository for PgNoteRepository {
        async fn create_note(
            &self,
            user_id: i64,
            title: &str,
            content: &str,
        ) -> Result<Note, RepoError> {
            let inserted = sqlx::query_as::<sqlx::Postgres, Note>(
                r#"INSERT INTO notes (user_id, title, content, created_at, updated_at)
                   VALUES ($1, $2, $3, NOW(), NOW())
                   RETURNING id,
                             user_id as author_id,
                             title,
                             content,
                             EXTRACT(EPOCH FROM created_at)::bigint as created_at,
                             EXTRACT(EPOCH FROM updated_at)::bigint as updated_at"#,
            )
            .bind(user_id)
            .bind(title)
            .bind(content)
            .fetch_one(&self.pool)
            .await
            .map_err(RepoError::DbError)?;
            Ok(inserted)
        }

        async fn find_by_id(&self, note_id: i64) -> Result<Option<Note>, RepoError> {
            let note = sqlx::query_as::<sqlx::Postgres, Note>(
                r#"SELECT id,
                          user_id as author_id,
                          title,
                          content,
                          EXTRACT(EPOCH FROM created_at)::bigint as created_at,
                          EXTRACT(EPOCH FROM updated_at)::bigint as updated_at
                   FROM notes WHERE id = $1"#,
            )
            .bind(note_id)
            .fetch_optional(&self.pool)
            .await
            .map_err(RepoError::DbError)?;
            Ok(note)
        }

        async fn update_note(
            &self,
            note_id: i64,
            user_id: i64,
            title: Option<&str>,
            content: Option<&str>,
        ) -> Result<Option<Note>, RepoError> {
            let updated = sqlx::query_as::<sqlx::Postgres, Note>(
                r#"UPDATE notes
                   SET title = COALESCE($1, title),
                       content = COALESCE($2, content),
                       updated_at = NOW()
                   WHERE id = $3 AND user_id = $4
                   RETURNING id,
                             user_id as author_id,
                             title,
                             content,
                             EXTRACT(EPOCH FROM created_at)::bigint as created_at,
                             EXTRACT(EPOCH FROM updated_at)::bigint as updated_at"#,
            )
            .bind(title)
            .bind(content)
            .bind(note_id)
            .bind(user_id)
            .fetch_optional(&self.pool)
            .await
            .map_err(RepoError::DbError)?;
            Ok(updated)
        }

        async fn delete_note(&self, note_id: i64, user_id: i64) -> Result<bool, RepoError> {
            let res = sqlx::query::<sqlx::Postgres>(
                r#"DELETE FROM notes WHERE id = $1 AND user_id = $2"#,
            )
            .bind(note_id)
            .bind(user_id)
            .execute(&self.pool)
            .await
            .map_err(RepoError::DbError)?;
            Ok(res.rows_affected() > 0)
        }

        async fn list_notes(&self) -> Result<Vec<Note>, RepoError> {
            let notes = sqlx::query_as::<sqlx::Postgres, Note>(
                r#"SELECT id,
                          user_id as author_id,
                          title,
                          content,
                          EXTRACT(EPOCH FROM created_at)::bigint as created_at,
                          EXTRACT(EPOCH FROM updated_at)::bigint as updated_at
                   FROM notes ORDER BY created_at DESC"#,
            )
            .fetch_all(&self.pool)
            .await
            .map_err(RepoError::DbError)?;
            Ok(notes)
        }
    }
}
