use actix_web::{App, HttpServer};
use dotenvy::dotenv;

mod api;
mod app;
mod model;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    HttpServer::new(|| App::new())
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
