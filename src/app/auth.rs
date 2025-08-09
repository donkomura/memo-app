use actix_web::{post, web, HttpResponse, Responder};
use std::sync::Arc;

use crate::app::model::{SignupInput, LoginInput, LoginOutput};
use crate::service::auth::AuthService;
use crate::middleware::auth::token::JwtTokenService;
use crate::middleware::auth::extractor::AuthenticatedUser;

#[post("/auth/signup")]
pub async fn signup(
    auth_service: web::Data<Arc<dyn AuthService>>,
    payload: web::Json<SignupInput>,
) -> impl Responder {
    match auth_service.signup(&payload.email, &payload.password).await {
        Ok(Some(_user)) => HttpResponse::Created().finish(),
        Ok(None) => HttpResponse::Conflict().finish(),
        Err(crate::service::auth::AuthServiceError::InvalidEmail | crate::service::auth::AuthServiceError::InvalidPassword) => {
            HttpResponse::BadRequest().finish()
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[post("/me")]
pub async fn me(user: AuthenticatedUser) -> impl Responder {
    HttpResponse::Ok().json(user.0)
}

#[post("/auth/login")]
pub async fn login(
    auth_service: web::Data<Arc<dyn AuthService>>,
    jwt: web::Data<JwtTokenService>,
    payload: web::Json<LoginInput>,
) -> impl Responder {
    match auth_service.login(&payload.email, &payload.password).await {
        Ok(Some(user)) => match jwt.generate(user.id) {
            Ok(token) => HttpResponse::Ok().json(LoginOutput { token }),
            Err(_) => HttpResponse::InternalServerError().finish(),
        },
        Err(crate::service::auth::AuthServiceError::InvalidCredentials) => {
            HttpResponse::Unauthorized().finish()
        }
        _ => HttpResponse::InternalServerError().finish(),
    }
}
