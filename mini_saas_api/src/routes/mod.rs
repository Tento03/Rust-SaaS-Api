use crate::handlers::health_handler;
use axum::{routing::get, Router};

pub fn create_routes() -> Router {
    Router::new()
        .route("/", get(health_handler::root))
        .route("/health", get(health_handler::health))
}
