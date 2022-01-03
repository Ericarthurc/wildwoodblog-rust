use axum::http::StatusCode;
use axum::response::{IntoResponse, Json, Response};
use serde_json::json;

#[derive(Debug)]
pub enum AppError {
    Io(std::io::Error),
    Empty(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::Io(error) => (StatusCode::UNPROCESSABLE_ENTITY, error.to_string()),
            AppError::Empty(error) => (StatusCode::UNPROCESSABLE_ENTITY, error),
        };

        let body = Json(json!({ "error": error_message }));

        (status, body).into_response()
    }
}

impl From<std::io::Error> for AppError {
    fn from(inner: std::io::Error) -> Self {
        AppError::Io(inner)
    }
}
