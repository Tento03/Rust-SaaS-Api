use axum::{routing::get, Router};

use crate::handlers::health_handler;

pub fn create_routes() -> Router {
    return Router::new()
        .route("/", get(health_handler::root))
        .route("/health", get(health_handler::health));
}
