use crate::models::api_response::ApiResponse;
use axum::Json;

pub async fn root() -> Json<ApiResponse> {
    Json(ApiResponse {
        message: "Mini SaaS API running 🚀".to_string(),
        status: "success".to_string(),
    })
}

pub async fn health() -> Json<ApiResponse> {
    Json(ApiResponse {
        message: "Service healthy".to_string(),
        status: "ok".to_string(),
    })
}
