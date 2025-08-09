mod app;
mod repository;

use actix_web::{web, App, HttpServer};
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use std::sync::Arc;

use app::auth::signup;
use repository::user::{SqliteUserRepository, UserRepository};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool: SqlitePool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("db connect"); 

    let user_repo: Arc<dyn UserRepository> = Arc::new(SqliteUserRepository::new(pool.clone()));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(user_repo.clone()))
            .service(signup)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
