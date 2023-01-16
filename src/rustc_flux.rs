use std::{
    ffi::OsStr,
    fmt,
    io::{self, BufRead},
    process::{Output, Stdio},
};

use serde::Deserialize;
use tokio::io::AsyncWriteExt;

#[derive(Default)]
pub struct RustcFluxOpts {
    error_format: ErrorFormat,
    crate_type: CrateType,
}

#[derive(Default)]
pub enum ErrorFormat {
    #[default]
    Human,
    Json,
    Short,
}

#[derive(Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum CrateType {
    #[default]
    Bin,
    Rlib,
}

#[derive(Deserialize, Debug)]
pub struct RustcError {
    pub message: String,
    pub code: Option<Code>,
    pub level: ErrorLevel,
    pub spans: Vec<Span>,
    pub children: Vec<RustcError>,
}

#[derive(Deserialize, Debug)]
pub struct Code {
    pub code: String,
    pub explanation: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Span {
    pub line_start: u64,
    pub line_end: u64,
    pub column_start: u64,
    pub column_end: u64,
    pub is_primary: bool,
    pub label: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum ErrorLevel {
    Error,
    Warning,
    FailureNote,
    Help,
    Note,
}

pub async fn run(
    rustc_flux_path: impl AsRef<OsStr>,
    code: &str,
    opts: RustcFluxOpts,
) -> io::Result<Output> {
    let mut child = tokio::process::Command::new(rustc_flux_path)
        .stdin(Stdio::piped())
        .stderr(Stdio::piped())
        .arg("-Zcrate-attr=feature(register_tool)")
        .arg("-Zcrate-attr=register_tool(flux)")
        .arg(format!("--error-format={}", opts.error_format))
        .arg(format!("--crate-type={}", opts.crate_type))
        .arg("-")
        .spawn()?;

    child
        .stdin
        .take()
        .unwrap()
        .write_all(code.as_bytes())
        .await?;

    child.wait_with_output().await
}

impl RustcFluxOpts {
    pub fn error_format(mut self, error_format: ErrorFormat) -> Self {
        self.error_format = error_format;
        self
    }

    pub fn crate_type(mut self, crate_type: CrateType) -> Self {
        self.crate_type = crate_type;
        self
    }
}

pub fn parse_stderr_json(stderr: &[u8]) -> serde_json::Result<Vec<RustcError>> {
    let mut errors = vec![];
    for line in stderr.lines() {
        let line = line.unwrap();
        errors.push(serde_json::from_str(&line)?)
    }
    Ok(errors)
}

impl fmt::Display for ErrorFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorFormat::Human => write!(f, "human"),
            ErrorFormat::Json => write!(f, "json"),
            ErrorFormat::Short => write!(f, "short"),
        }
    }
}

impl fmt::Display for CrateType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CrateType::Rlib => write!(f, "rlib"),
            CrateType::Bin => write!(f, "bin"),
        }
    }
}
