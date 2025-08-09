use actix_web::{post, web, HttpResponse, Responder};

use crate::app::model::SignupInput;

#[post("/auth/signup")]
pub async fn signup() -> impl Responder {
    HttpResponse::Ok()
}
