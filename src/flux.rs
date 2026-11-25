use std::{
    fmt,
    io::{self, BufRead},
    path::PathBuf,
    process::{Output, Stdio},
    time::Duration,
};

const TIMEOUT: Duration = Duration::from_secs(60);

use process_wrap::tokio::{ChildWrapper, CommandWrap, ProcessGroup};
use serde::{Deserialize, Serialize};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

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
        let mut command = CommandWrap::with_new(&self.flux_path, |command| {
            if let Some(working_dir) = &self.working_dir {
                command.current_dir(working_dir);
            }
            command
                .stdin(Stdio::piped())
                .stderr(Stdio::piped())
                .arg("-Fsummary=off")
                .arg(format!("--error-format={}", self.error_format))
                .arg(format!("--crate-type={}", self.crate_type))
                .arg(format!("--color={}", self.color))
                .arg("--edition=2024")
                .arg("-");
        });
        // Run flux as the leader of a new process group, so that it and everything it starts
        // (flux-driver, fixpoint, z3) can be killed together.
        command.wrap(ProcessGroup::leader());

        // Kills the process group unless flux finishes: on timeout, on an I/O error, or when the
        // request is cancelled (e.g. the client disconnects and this future is dropped).
        let mut child = KillGroupOnDrop::new(command.spawn()?);

        child
            .child
            .stdin()
            .take()
            .unwrap()
            .write_all(code.as_bytes())
            .await?;

        let mut stderr_pipe = child.child.stderr().take().unwrap();
        let wait = async {
            let mut stderr = Vec::new();
            let (status, _) =
                tokio::try_join!(child.child.wait(), stderr_pipe.read_to_end(&mut stderr))?;
            io::Result::Ok(Output {
                status,
                stdout: Vec::new(),
                stderr,
            })
        };
        let output = tokio::time::timeout(TIMEOUT, wait).await??;
        child.disarm();
        Ok(output)
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

/// Kills a child's whole process group when dropped, unless disarmed.
///
/// `flux` is only a launcher: it starts `flux-driver`, which starts `fixpoint`, which starts `z3`.
/// process-wrap's `KillOnDrop` only sets tokio's `kill_on_drop`, which kills just `flux` and would
/// leave the rest running. For a child spawned with `ProcessGroup`, `start_kill` signals the whole
/// group instead.
struct KillGroupOnDrop {
    child: Box<dyn ChildWrapper>,
    armed: bool,
}

impl KillGroupOnDrop {
    fn new(child: Box<dyn ChildWrapper>) -> Self {
        KillGroupOnDrop { child, armed: true }
    }

    fn disarm(&mut self) {
        self.armed = false;
    }
}

impl Drop for KillGroupOnDrop {
    fn drop(&mut self) {
        if self.armed {
            // Errors (e.g. the group already exited) are ignored.
            let _ = self.child.start_kill();
        }
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
