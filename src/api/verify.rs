use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use serde_repr::Serialize_repr;

use crate::{
    flux::{self, CrateType, ErrorLevel, Flux, RustcError},
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
    Success,
    /// The program contains Rust or Flux errors
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
    let markers = map_errors_to_markers(errors);
    Ok(Json(VerifyRes::new(status, markers)))
}

impl VerifyRes {
    fn new(status: Status, markers: Vec<MarkerData>) -> Self {
        Self { status, markers }
    }
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
            ErrorLevel::ICE => Ok(MarkerSeverity::Error),
            ErrorLevel::Error => Ok(MarkerSeverity::Error),
            ErrorLevel::Warning => Ok(MarkerSeverity::Warning),
            ErrorLevel::Help => Ok(MarkerSeverity::Hint),
            ErrorLevel::Note => Ok(MarkerSeverity::Info),
            ErrorLevel::FailureNote => Err(()),
        }
    }
}
