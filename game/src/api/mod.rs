use actix_web::{
    get,
    web::{Data, Path},
    HttpResponse, Responder,
};

use crate::{
    app::AppState,
    model::db::{Consumable, SessionRow},
};

mod action;

#[get("/{session}")]
pub async fn get_game_state(path: Path<i32>, state: Data<AppState>) -> impl Responder {
    let session_id = path.into_inner();
    match sqlx::query_as!(
        SessionRow,
        r#"SELECT session, money, ante, rounds_completed, rounds_skipped, hands, discards, hand_size, last_consumable as "last_consumable: Consumable", high_card_level, pair_level, two_pair_level, three_of_a_kind_level, straight_level, flush_level, full_house_level, four_of_a_kind_level, straight_flush_level, five_of_a_kind_level, flush_house_level, flush_five_level, high_card_played, pair_played, two_pair_played, three_of_a_kind_played, straight_played, flush_played, full_house_played, four_of_a_kind_played, straight_flush_played, five_of_a_kind_played, flush_house_played, flush_five_played, pluto_used, mercury_used, uranus_used, venus_used, saturn_used, jupiter_used, earth_used, mars_used, neptune_used, planet_x_used, ceres_used, eris_used, overstock_redeemed, overstock_plus_redeemed, clearance_sale_redeemed, liquidation_redeemed, hone_redeemed, glow_up_redeemed, reroll_surplus_redeemed, reroll_glut_redeemed, crystal_ball_redeemed, omen_globe_redeemed, telescope_redeemed, observatory_redeemed, grabber_redeemed, nacho_tong_redeemed, wasteful_redeemed, recyclomancy_redeemed, tarot_merchant_redeemed, tarot_tycoon_redeemed, planet_merchant_redeemed, planet_tycoon_redeemed, seed_money_redeemed, money_tree_redeemed, blank_redeemed, antimatter_redeemed, magic_trick_redeemed, illusion_redeemed, hieroglyph_redeemed, petroglyph_redeemed, directors_cut_redeemed, retcon_redeemed, paint_brush_redeemed, palette_redeemed FROM sessions WHERE session = $1"#,
        session_id
    ).fetch_one(&state.db).await {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(_) => HttpResponse::InternalServerError().body("Unable to execute query"),
    }
}
