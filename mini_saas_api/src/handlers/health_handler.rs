use crate::models::api_response::ApiResponse;
use axum::Json;

pub async fn root() -> Json<ApiResponse> {
    return Json(ApiResponse {
        message: ("Mini Saas API running".to_string()),
        status: ("success".to_string()),
    });
}

pub async fn health() -> Json<ApiResponse> {
    return Json(ApiResponse {
        message: ("Service healthy".to_string()),
        status: ("ok".to_string()),
    });
}
