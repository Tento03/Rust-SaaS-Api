use std::net::SocketAddr;

use dotenvy::dotenv;

mod config;
mod handlers;
mod models;
mod routes;

#[derive(Clone)]
pub struct AppState {
    pub db: sqlx::MySqlPool,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    tracing_subscriber::fmt::init();

    let pool = config::database::init_db().await;

    let state = AppState { db: pool };

    let app = routes::health_routes::create_routes(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    println!("Server running at http://{}", addr);

    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}
