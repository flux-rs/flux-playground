use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};

use crate::{
    flux::{self, CrateType, Flux},
    AppError, AppState,
};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VerifyReq {
    code: String,
    crate_type: CrateType,
}

#[derive(Serialize)]
pub struct VerifyRes {
    status: Status,
    errors: Vec<flux::RustcError>,
}

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    /// No errors
    Success,
    /// The program contains Rust or Flux errors
    Error,
}

pub async fn verify(
    State(state): State<AppState>,
    Json(req): Json<VerifyReq>,
) -> Result<Json<VerifyRes>, AppError> {
    let output = Flux::new(state.flux_path)
        .error_format(flux::ErrorFormat::Json)
        .crate_type(req.crate_type)
        // Run inside examples/lib to be able to declare modules
        // there without specifying a path.
        .working_dir(state.examples.join("lib"))
        .run(&req.code)
        .await?;

    let errors = flux::parse_stderr_json(&output.stderr)?;
    let status = if output.status.success() {
        Status::Success
    } else {
        Status::Error
    };
    Ok(Json(VerifyRes::new(status, errors)))
}

impl VerifyRes {
    fn new(status: Status, errors: Vec<flux::RustcError>) -> Self {
        Self { status, errors }
    }
}
