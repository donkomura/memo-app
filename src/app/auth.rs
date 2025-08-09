use actix_web::{post, web, HttpResponse, Responder};
use argon2::{Argon2, PasswordHasher};
use password_hash::{rand_core::OsRng, SaltString};
use std::sync::Arc;

use crate::app::model::SignupInput;
use crate::repository::user::UserRepository;

#[post("/auth/signup")]
pub async fn signup(
    user_repo: web::Data<Arc<dyn UserRepository>>,
    payload: web::Json<SignupInput>,
) -> impl Responder {
    let salt = SaltString::generate(&mut OsRng);
    let hash = match Argon2::default().hash_password(payload.password.as_bytes(), &salt) {
        Ok(h) => h.to_string(),
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    match user_repo.create_user(&payload.email, &hash).await {
        Ok(true) => HttpResponse::Created().finish(),
        Ok(false) => HttpResponse::Conflict().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
