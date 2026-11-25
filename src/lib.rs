use std::{path::PathBuf, sync::Arc};

use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

pub mod api;
pub mod flux;
pub mod limiter;
pub mod play_rust_lang_org;

#[derive(Clone)]
pub struct AppState {
    pub flux_path: PathBuf,
    pub ansi_to_html: bool,
    pub examples: PathBuf,
    pub limiter: Arc<limiter::Limiter>,
}

pub struct AppError {
    status: StatusCode,
    descr: String,
}

impl AppError {
    /// Too many requests are already running or queued.
    pub fn busy() -> Self {
        AppError {
            status: StatusCode::SERVICE_UNAVAILABLE,
            descr: "The server is busy, please try again later".to_string(),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let body = json!({"error": self.descr});
        (self.status, Json(body)).into_response()
    }
}

impl<T: std::error::Error> From<T> for AppError {
    fn from(err: T) -> Self {
        AppError {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            descr: err.to_string(),
        }
    }
}
