#[derive(Error, debug)]
pub enum AppError {
    #[error("Database error")]
    DatabaseError,

    #[Error("Task not found")]
    NotFound,
}

#[derive(Serialize)]
struct ErrorResponse {
    message: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = match self {
            AppError::DatabaseError => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::NotFound => StatusCode::NotFound,
        };

        let body = Json(ErrorResponse {
            message: self.to_string(),
        });

        (status, body).into_response()
    }
}
