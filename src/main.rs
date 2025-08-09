mod app;
mod repository;
mod service;
mod domain;
mod middleware;

use actix_web::{web, App, HttpServer};
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use std::sync::Arc;

use app::auth::{signup, login, me};
use app::notes::{get_note, create_note, update_note};
use repository::user::{SqliteUserRepository, UserRepository};
use repository::note::{SqliteNoteRepository, NoteRepository};
use service::auth::{AuthService, AuthServiceImpl};
use middleware::auth::token::JwtTokenService;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool: SqlitePool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("db connect"); 

    let user_repo: Arc<dyn UserRepository> = Arc::new(SqliteUserRepository::new(pool.clone()));
    let note_repo: Arc<dyn NoteRepository> = Arc::new(SqliteNoteRepository::new(pool.clone()));
    let auth_service: Arc<dyn AuthService> = Arc::new(AuthServiceImpl::new(user_repo.clone()));
    let jwt = web::Data::new(JwtTokenService::from_env().expect("JWT config"));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(auth_service.clone()))
            .app_data(web::Data::new(note_repo.clone()))
            .app_data(jwt.clone())
            .service(signup)
            .service(login)
            .service(me)
            .service(get_note)
            .service(create_note)
            .service(update_note)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
