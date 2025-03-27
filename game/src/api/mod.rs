use actix_web::{get, Responder};

mod action;

#[get("/")]
pub async fn get_game_state() -> impl Responder {
    todo!()
}
