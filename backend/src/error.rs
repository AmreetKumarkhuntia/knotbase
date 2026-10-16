use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;

// shortcut: axum extractor rejections are plain text, not {"error"} (malformed JSON 400, missing/mistyped field 422, wrong content-type 415, /api/nodes/abc 400, wrong verb on a known path empty 405); wrap Json/Path in a FromRequest that maps to AppError when a client must parse them.
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
            // CHECK / FK / NOT NULL / UNIQUE violations: the schema is the validator
            sqlx::Error::Database(ref d) if d.kind() != sqlx::error::ErrorKind::Other => {
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
