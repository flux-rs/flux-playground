use std::path::PathBuf;

use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use serde_repr::Serialize_repr;

use crate::{
    rustc_flux::{self, CrateType, ErrorLevel, RustcError, RustcFluxOpts},
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
    markers: Vec<MarkerData>,
}

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    /// No errors
    Safe,
    /// The program contains flux errors
    Unsafe,
    /// The program contains rust errors
    Error,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MarkerData {
    severity: MarkerSeverity,
    message: String,
    start_line_number: u64,
    start_column: u64,
    end_line_number: u64,
    end_column: u64,
}

#[derive(Serialize_repr)]
#[repr(u8)]
pub enum MarkerSeverity {
    Hint = 1,
    Info = 2,
    Warning = 4,
    Error = 8,
}

pub async fn verify(
    State(state): State<AppState>,
    Json(req): Json<VerifyReq>,
) -> Result<Json<VerifyRes>, AppError> {
    let opts = RustcFluxOpts::default()
        .error_format(rustc_flux::ErrorFormat::Json)
        .crate_type(req.crate_type);
    let output = rustc_flux::run(state.rustc_flux, &req.code, opts).await?;

    let errors = rustc_flux::parse_stderr_json(&output.stderr)?;
    let status = if output.status.success() {
        Status::Safe
    } else if has_flux_errors(&errors) {
        Status::Unsafe
    } else {
        Status::Error
    };
    let markers = map_errors_to_markers(errors);
    Ok(Json(VerifyRes::new(status, markers)))
}

impl VerifyRes {
    fn new(status: Status, markers: Vec<MarkerData>) -> Self {
        Self { status, markers }
    }
}

fn has_flux_errors(errors: &[RustcError]) -> bool {
    errors
        .iter()
        .filter_map(|err| err.code.as_ref())
        .any(|code| code.code == "FLUX")
}

fn map_errors_to_markers(errors: Vec<RustcError>) -> Vec<MarkerData> {
    errors
        .into_iter()
        .flat_map(|error| {
            let primary_span = error.spans.into_iter().find(|span| span.is_primary)?;

            let mut message = error.message;
            if let Some(label) = primary_span.label {
                message = format!("{message}\n{label}");
            }

            let marker = MarkerData {
                severity: error.level.try_into().ok()?,
                message,
                start_line_number: primary_span.line_start,
                start_column: primary_span.column_start,
                end_line_number: primary_span.line_end,
                end_column: primary_span.column_end,
            };

            Some(marker)
        })
        .collect()
}

impl TryFrom<ErrorLevel> for MarkerSeverity {
    type Error = ();

    fn try_from(level: ErrorLevel) -> Result<Self, ()> {
        match level {
            ErrorLevel::Error => Ok(MarkerSeverity::Error),
            ErrorLevel::Warning => Ok(MarkerSeverity::Warning),
            ErrorLevel::Help => Ok(MarkerSeverity::Hint),
            ErrorLevel::Note => Ok(MarkerSeverity::Info),
            ErrorLevel::FailureNote => Err(()),
        }
    }
}
