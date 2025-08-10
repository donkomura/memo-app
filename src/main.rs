mod app;
mod domain;
mod middleware;
mod repository;
mod service;

use actix_web::{App, HttpServer, web};
#[cfg(feature = "postgres")]
use sqlx::{PgPool, postgres::PgPoolOptions};
#[cfg(not(feature = "postgres"))]
use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};
use std::sync::Arc;

use app::auth::{login, me, signup};
use app::notes::{create_note, delete_note, get_note, list_notes, update_note};
use middleware::auth::token::JwtTokenService;
use repository::note::NoteRepository;
use repository::user::UserRepository;
#[cfg(feature = "postgres")]
use repository::{note::PgNoteRepository, user::PgUserRepository};
#[cfg(not(feature = "postgres"))]
use repository::{note::SqliteNoteRepository, user::SqliteUserRepository};
use service::auth::{AuthService, AuthServiceImpl};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool: AppPool = create_pool(&database_url).await;

    let (user_repo, note_repo) = create_repositories(pool.clone());
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
            .service(delete_note)
            .service(list_notes)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

#[cfg(feature = "postgres")]
type AppPool = PgPool;
#[cfg(not(feature = "postgres"))]
type AppPool = SqlitePool;

#[cfg(feature = "postgres")]
async fn create_pool(database_url: &str) -> AppPool {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
        .expect("db connect")
}

#[cfg(not(feature = "postgres"))]
async fn create_pool(database_url: &str) -> AppPool {
    SqlitePoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
        .expect("db connect")
}

#[cfg(feature = "postgres")]
fn create_repositories(pool: AppPool) -> (Arc<dyn UserRepository>, Arc<dyn NoteRepository>) {
    (
        Arc::new(PgUserRepository::new(pool.clone())) as Arc<dyn UserRepository>,
        Arc::new(PgNoteRepository::new(pool)) as Arc<dyn NoteRepository>,
    )
}

#[cfg(not(feature = "postgres"))]
fn create_repositories(pool: AppPool) -> (Arc<dyn UserRepository>, Arc<dyn NoteRepository>) {
    (
        Arc::new(SqliteUserRepository::new(pool.clone())) as Arc<dyn UserRepository>,
        Arc::new(SqliteNoteRepository::new(pool)) as Arc<dyn NoteRepository>,
    )
}
