use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;

pub enum AppError {
    BadRequest(String),
    NotFound,
    Internal(sqlx::Error),
}
pub type Result<T> = std::result::Result<T, AppError>;

impl From<sqlx::Error> for AppError {
    fn from(e: sqlx::Error) -> Self {
        match e {
            sqlx::Error::RowNotFound => Self::NotFound,
            // CHECK / FK / NOT NULL / UNIQUE violations: the schema is the validator. SQLSTATE class 22 is Postgres'
            // "data exception" (a NUL byte in text, a value out of range); SQLite's codes are numeric and never start with 22.
            sqlx::Error::Database(ref d)
                if d.kind() != sqlx::error::ErrorKind::Other
                    || d.code().is_some_and(|c| c.starts_with("22")) =>
            {
                Self::BadRequest(d.message().to_string())
            }
            e => Self::Internal(e),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, msg) = match self {
            Self::BadRequest(m) => (StatusCode::BAD_REQUEST, m),
            Self::NotFound => (StatusCode::NOT_FOUND, "not found".into()),
            Self::Internal(e) => {
                eprintln!("db error: {e}");
                (StatusCode::INTERNAL_SERVER_ERROR, "internal error".into())
            }
        };
        (status, Json(json!({ "error": msg }))).into_response()
    }
}
