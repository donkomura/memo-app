use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct SignupInput {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginInput {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginOutput {
    pub token: String, // JWT
}

#[derive(Deserialize)]
pub struct CreateNoteInput {
    pub title: String,
    pub content: String,
}

#[derive(Deserialize)]
pub struct UpdateNoteInput {
    pub title: Option<String>,
    pub content: Option<String>,
}
