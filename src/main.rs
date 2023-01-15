use serde_json::json;
use std::{net::SocketAddr, path::PathBuf, process::Stdio};
use tokio::io::AsyncWriteExt;
use tower_http::cors::{self, CorsLayer};

use axum::{
    extract::State,
    http::{header, Method, StatusCode},
    response::IntoResponse,
    routing::post,
    Json, Router,
};
use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Parser, Clone)]
struct Args {
    #[arg(short, long, default_value_t = 8888)]
    port: u16,
    #[arg(long)]
    rustc_flux_path: PathBuf,
}

struct AppError {
    descr: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let addr = SocketAddr::from(([0, 0, 0, 0], args.port));

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([header::CONTENT_TYPE])
        .allow_origin(cors::Any);

    let app = Router::new()
        .route("/meta/crates", post(crates))
        .route("/evaluate.json", post(evaluate))
        .layer(cors)
        .with_state(args);

    println!("listening on {addr}");

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn crates() -> Json<serde_json::Value> {
    let body = json!({"crates": []});
    Json(body)
}

#[derive(Deserialize)]
struct EvaluateReq {
    code: String,
}

#[derive(Serialize)]
struct EvaluateRes {
    result: String,
    error: Option<String>,
}

impl EvaluateRes {
    fn err(err: impl ToString) -> Self {
        let str = err.to_string();
        Self {
            result: str.clone(),
            error: Some(str),
        }
    }

    fn success(result: impl ToString) -> Self {
        Self {
            result: result.to_string(),
            error: None,
        }
    }
}

async fn evaluate(
    State(args): State<Args>,
    Json(req): Json<EvaluateReq>,
) -> Result<Json<EvaluateRes>, AppError> {
    let mut child = tokio::process::Command::new(args.rustc_flux_path)
        .stdin(Stdio::piped())
        .stderr(Stdio::piped())
        .arg("-Zcrate-attr=feature(register_tool)")
        .arg("-Zcrate-attr=register_tool(flux)")
        .arg("-")
        .spawn()?;

    child
        .stdin
        .take()
        .unwrap()
        .write(req.code.as_bytes())
        .await?;

    let out = child.wait_with_output().await?;

    let res = if out.status.success() {
        EvaluateRes::success("Safe")
    } else {
        EvaluateRes::err(String::from_utf8(out.stderr)?)
    };
    Ok(Json(res))
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
