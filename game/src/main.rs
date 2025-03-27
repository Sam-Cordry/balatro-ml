use actix_web::{App, HttpServer};

mod api;
mod game;
mod model;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new())
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
