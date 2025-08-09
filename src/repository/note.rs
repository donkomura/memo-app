use crate::domain::model::Note;
use crate::repository::user::RepoError;

#[async_trait::async_trait]
pub trait NoteRepository: Send + Sync + 'static {
    async fn create_note(&self, user_id: i64, title: &str, content: &str) -> Result<Note, RepoError>;
    async fn find_by_id(&self, note_id: i64) -> Result<Option<Note>, RepoError>;
    async fn update_note(
        &self,
        note_id: i64,
        user_id: i64,
        title: Option<&str>,
        content: Option<&str>,
    ) -> Result<Option<Note>, RepoError>;
}

use sqlx::SqlitePool;

pub struct SqliteNoteRepository {
    pub(crate) pool: SqlitePool,
}

impl SqliteNoteRepository {
    pub fn new(pool: SqlitePool) -> Self { Self { pool } }
}

#[async_trait::async_trait]
impl NoteRepository for SqliteNoteRepository {
    async fn create_note(&self, user_id: i64, title: &str, content: &str) -> Result<Note, RepoError> {
        let inserted = sqlx::query_as!(
            Note,
            r#"INSERT INTO notes (user_id, title, content, created_at, updated_at)
               VALUES (?, ?, ?, strftime('%s','now'), strftime('%s','now'))
               RETURNING id as "id!: i64", user_id as "author_id!: i64", title, content, created_at as "created_at!: i64", updated_at as "updated_at!: i64""#,
            user_id,
            title,
            content
        )
        .fetch_one(&self.pool)
        .await
        .map_err(RepoError::DbError)?;

        Ok(inserted)
    }
    async fn find_by_id(&self, note_id: i64) -> Result<Option<Note>, RepoError> {
        let note = sqlx::query_as!(
            Note,
            r#"SELECT id as "id!: i64", user_id as "author_id!: i64", title, content, created_at as "created_at!: i64", updated_at as "updated_at!: i64"
               FROM notes
               WHERE id = ?"#,
            note_id
        )
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
        let updated = sqlx::query_as!(
            Note,
            r#"UPDATE notes
               SET title = COALESCE(?, title),
                   content = COALESCE(?, content),
                   updated_at = strftime('%s','now')
               WHERE id = ? AND user_id = ?
               RETURNING id as "id!: i64", user_id as "author_id!: i64", title, content, created_at as "created_at!: i64", updated_at as "updated_at!: i64""#,
            title,
            content,
            note_id,
            user_id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(RepoError::DbError)?;

        Ok(updated)
    }
}


