use std::path::PathBuf;

use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

pub mod api;
pub mod flux;
pub mod play_rust_lang_org;

#[derive(Clone)]
pub struct AppState {
    pub flux_path: PathBuf,
    pub ansi_to_html: bool,
    pub examples: PathBuf,
}

pub struct AppError {
    descr: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let body = json!({"error": self.descr});
        (StatusCode::INTERNAL_SERVER_ERROR, Json(body)).into_response()
    }
}

impl<T: std::error::Error> From<T> for AppError {
    fn from(err: T) -> Self {
        AppError {
            descr: err.to_string(),
        }
    }
}
