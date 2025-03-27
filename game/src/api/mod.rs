use actix_web::{get, HttpResponse, Responder};

mod action;

#[get("/")]
pub async fn get_game_state() -> impl Responder {
    HttpResponse::NotImplemented()
}
