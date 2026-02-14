use axum::{routing::get, Router};
use std::net::SocketAddr;
use tracing_subscriber;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(root))
        .route("/health", get(health));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    println!("🚀 Server running at http://{}", addr);

    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "Mini SaaS API is running 🚀"
}

async fn health() -> &'static str {
    "OK"
}
