use serde::Serialize;

#[derive(sqlx::FromRow, Debug, Clone)]
pub struct User {
    pub id: i64,
    pub email: String,
    pub password_hash: String, // Argon2id PHC string
    pub created_at: i64,
}

#[derive(sqlx::FromRow, Debug, Clone, Serialize)]
pub struct Note {
    pub id: i64,
    pub user_id: i64,
    pub title: String,
    pub content: String,
    pub created_at: i64,
    pub updated_at: i64,
}

