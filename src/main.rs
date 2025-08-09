mod app;

use actix_web::{web, App, HttpServer};
use app::auth::signup;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .service(signup)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
