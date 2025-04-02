use actix_web::web::Data;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub struct AppState {
    pub db: Pool<Postgres>,
}

pub async fn get_app_data() -> Data<AppState> {
    let db_pool = PgPoolOptions::new()
        .connect(&dotenvy::var("DATABASE_URL").expect("DATABASE_URL not set"))
        .await
        .expect("Connection error");
    Data::new(AppState { db: db_pool })
}
