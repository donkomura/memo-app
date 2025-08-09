use actix_web::{post, web, HttpResponse, Responder};
use std::sync::Arc;

use crate::app::model::SignupInput;
use crate::service::auth::AuthService;

#[post("/auth/signup")]
pub async fn signup(
    auth_service: web::Data<Arc<dyn AuthService>>,
    payload: web::Json<SignupInput>,
) -> impl Responder {
    match auth_service.signup(&payload.email, &payload.password).await {
        Ok(true) => HttpResponse::Created().finish(),
        Ok(false) => HttpResponse::Conflict().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
