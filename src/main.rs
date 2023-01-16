use flux_playground::{api, play_rust_lang_org};
use std::{
    net::{IpAddr, SocketAddr},
    path::PathBuf,
};
use tower_http::cors::{self, CorsLayer};

use axum::{
    http::{header, Method},
    routing::post,
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
    rustc_flux_path: PathBuf,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([header::CONTENT_TYPE])
        .allow_origin(cors::Any);

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
        .layer(cors)
        .with_state(args.rustc_flux_path);

    let addr = SocketAddr::from((args.bind, args.port));

    println!("Listening on {addr} ...");

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
