///! Mock endpoints of play.rust-lang.org to allow running flux in mdbook
use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{rustc_flux, AppError, AppState};

pub async fn crates() -> Json<serde_json::Value> {
    let body = json!({"crates": []});
    Json(body)
}

#[derive(Deserialize)]
pub struct EvaluateReq {
    code: String,
}

#[derive(Serialize)]
pub struct EvaluateRes {
    result: String,
    error: Option<String>,
}

impl EvaluateRes {
    fn success(result: impl ToString) -> Self {
        Self {
            result: result.to_string(),
            error: None,
        }
    }

    fn error(err: impl ToString) -> Self {
        let str = err.to_string();
        Self {
            result: str.clone(),
            error: Some(str),
        }
    }
}

pub async fn evaluate(
    State(state): State<AppState>,
    Json(req): Json<EvaluateReq>,
) -> Result<Json<EvaluateRes>, AppError> {
    let out = rustc_flux::run(state.rustc_flux, &req.code, Default::default()).await?;

    let res = if out.status.success() {
        EvaluateRes::success("SAFE")
    } else {
        EvaluateRes::error(String::from_utf8(out.stderr)?)
    };
    Ok(Json(res))
}
