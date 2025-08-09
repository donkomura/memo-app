mod app;
mod repository;
mod service;

use actix_web::{web, App, HttpServer};
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use std::sync::Arc;

use app::auth::signup;
use repository::user::{SqliteUserRepository, UserRepository};
use service::auth::{AuthService, AuthServiceImpl};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool: SqlitePool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("db connect"); 

    let user_repo: Arc<dyn UserRepository> = Arc::new(SqliteUserRepository::new(pool.clone()));
    let auth_service: Arc<dyn AuthService> = Arc::new(AuthServiceImpl::new(user_repo.clone()));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(auth_service.clone()))
            .service(signup)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
