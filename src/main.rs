use flux_playground::{api, limiter::Limiter, play_rust_lang_org, AppError, AppState};
use std::{
    io,
    net::{IpAddr, SocketAddr},
    num::NonZeroUsize,
    path::PathBuf,
    sync::Arc,
    thread,
};
use tower_http::{
    cors::{self, CorsLayer},
    services::ServeDir,
};

use axum::{
    http::{header, Method},
    routing::{get, get_service, post},
    Router,
};
use clap::Parser;

#[derive(Parser, Clone)]
struct Args {
    #[arg(short, long, default_value_t = 3000)]
    port: u16,
    #[arg(long, default_value_t = IpAddr::from([0, 0, 0, 0]))]
    bind: IpAddr,
    #[arg(long)]
    flux_path: PathBuf,
    /// Run with `--color==awlays` and convert ANSI escape codes to HTML
    #[arg(long, default_value = "false")]
    ansi_to_html: bool,
    #[arg(long, default_value = "./static")]
    r#static: PathBuf,
    #[arg(long, default_value = "./examples")]
    examples: PathBuf,
    /// Maximum number of flux processes running at once [default: number of CPUs]
    #[arg(long)]
    max_concurrent: Option<NonZeroUsize>,
    /// Maximum number of requests waiting for a free slot; beyond that, requests get a 503
    #[arg(long, default_value_t = 32)]
    max_queued: usize,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let max_concurrent = args
        .max_concurrent
        .or_else(|| thread::available_parallelism().ok())
        .map_or(1, NonZeroUsize::get);
    let state = AppState {
        flux_path: args.flux_path,
        examples: args.examples.clone(),
        ansi_to_html: args.ansi_to_html,
        limiter: Arc::new(Limiter::new(max_concurrent, args.max_queued)),
    };

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([header::CONTENT_TYPE])
        .allow_origin(cors::Any);

    let serve_static = get_service(ServeDir::new(args.r#static)).handle_error(handle_error);
    let serve_examples = get_service(ServeDir::new(args.examples)).handle_error(handle_error);

    let app = Router::new()
        .route(
            "/play.rust-lang.org/meta/crates",
            post(play_rust_lang_org::crates),
        )
        .route(
            "/play.rust-lang.org/evaluate.json",
            post(play_rust_lang_org::evaluate),
        )
        .route("/api/verify", post(api::verify))
        .route("/api/examples", get(api::examples::list))
        .nest_service("/examples", serve_examples)
        .nest_service("/", serve_static)
        .layer(cors)
        .with_state(state);

    let addr = SocketAddr::from((args.bind, args.port));

    println!(
        "Listening on {addr} (max {max_concurrent} concurrent flux runs, {} queued) ...",
        args.max_queued
    );

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handle_error(err: io::Error) -> AppError {
    AppError::from(err)
}
