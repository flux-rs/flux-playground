///! Mock endpoints of play.rust-lang.org to allow running flux in mdbook
use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{
    rustc_flux::{Color, RustcFlux},
    AppError, AppState,
};

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

    fn error(err: String) -> Self {
        Self {
            result: err.clone(),
            error: Some(err),
        }
    }
}

pub async fn evaluate(
    State(state): State<AppState>,
    Json(req): Json<EvaluateReq>,
) -> Result<Json<EvaluateRes>, AppError> {
    let mut rustc = RustcFlux::new(state.rustc_flux);

    if state.ansi_to_html {
        rustc.color(Color::Always);
    }

    let out = rustc.run(&req.code).await?;

    let res = if out.status.success() {
        EvaluateRes::success("SAFE")
    } else {
        let stderr = String::from_utf8(out.stderr)?;
        if state.ansi_to_html {
            let html = ansi_to_html::convert_escaped(&stderr)?;
            EvaluateRes::error(html)
        } else {
            EvaluateRes::error(stderr)
        }
    };
    Ok(Json(res))
}
