use std::{
    fmt,
    io::{self, BufRead},
    path::PathBuf,
    process::{Output, Stdio},
};

use serde::{Deserialize, Serialize};
use tokio::io::AsyncWriteExt;

pub struct Flux {
    flux_path: PathBuf,
    error_format: ErrorFormat,
    crate_type: CrateType,
    color: Color,
    working_dir: Option<PathBuf>,
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

#[derive(Serialize, Deserialize, Debug)]
pub struct RustcError {
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<Code>,
    pub level: ErrorLevel,
    pub spans: Vec<Span>,
    pub children: Vec<RustcError>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Code {
    pub code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Span {
    pub line_start: u64,
    pub line_end: u64,
    pub column_start: u64,
    pub column_end: u64,
    pub is_primary: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
// #[serde(rename_all = "kebab-case")]
#[serde(try_from = "&str")]
pub enum ErrorLevel {
    Error,
    Warning,
    FailureNote,
    Help,
    Note,
    ICE,
}

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "lowercase")]
pub enum Color {
    #[default]
    Auto,
    Always,
    Never,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Color::Auto => write!(f, "auto"),
            Color::Always => write!(f, "always"),
            Color::Never => write!(f, "never"),
        }
    }
}

impl Flux {
    pub fn new(flux_path: PathBuf) -> Self {
        Flux {
            flux_path,
            error_format: Default::default(),
            crate_type: CrateType::Bin,
            color: Default::default(),
            working_dir: None,
        }
    }

    pub async fn run(&mut self, code: &str) -> io::Result<Output> {
        let mut command = tokio::process::Command::new(&self.flux_path);

        if let Some(working_dir) = &self.working_dir {
            command.current_dir(working_dir);
        }

        let mut child = command
            .stdin(Stdio::piped())
            .stderr(Stdio::piped())
            .env("FLUXFLAGS", "-Fsummary=off")
            .arg(format!("--error-format={}", self.error_format))
            .arg(format!("--crate-type={}", self.crate_type))
            .arg(format!("--color={}", self.color))
            .arg("--edition=2024")
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

    pub fn error_format(&mut self, error_format: ErrorFormat) -> &mut Self {
        self.error_format = error_format;
        self
    }

    pub fn crate_type(&mut self, crate_type: CrateType) -> &mut Self {
        self.crate_type = crate_type;
        self
    }

    pub fn color(&mut self, color: Color) -> &mut Self {
        self.color = color;
        self
    }

    pub fn working_dir(&mut self, path: PathBuf) -> &mut Self {
        self.working_dir = Some(path);
        self
    }
}

pub fn parse_stderr_json(stderr: &[u8]) -> serde_json::Result<Vec<RustcError>> {
    let mut errors = vec![];
    for line in stderr.lines() {
        let line = line.unwrap();
        // When there's an ICE the output contains lines that are not valid json, so we skip them.
        if line.starts_with('{') {
            errors.push(serde_json::from_str(&line)?)
        }
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

impl TryFrom<&str> for ErrorLevel {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, String> {
        let level = match value {
            "error" => ErrorLevel::Error,
            "warning" => ErrorLevel::Warning,
            "failure-note" => ErrorLevel::FailureNote,
            "help" => ErrorLevel::Help,
            "note" => ErrorLevel::Note,
            "error: internal compiler error" => ErrorLevel::ICE,
            _ => return Err(format!("unknown error level `{value}`")),
        };
        Ok(level)
    }
}
