mod handlers;
mod models;
mod routes;

use std::net::SocketAddr;
use tracing_subscriber;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = routes::create_routes();

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    println!("🚀 Server running at http://{}", addr);

    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}
