use std::env;

use sqlx::{mysql::MySqlPoolOptions, MySqlPool};

pub async fn init_db() -> MySqlPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE URL missing");

    return MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to DB");
}
