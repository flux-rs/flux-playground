use std::path::PathBuf;

use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use tokio::{fs, io::AsyncReadExt};

use crate::{AppError, AppState};

#[derive(Serialize, Deserialize)]
pub struct ListRes {
    groups: Vec<ExampleGroup>,
    examples: Vec<Example>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct ExampleGroup {
    id: String,
    display_name: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct Example {
    display_name: String,
    file_name: String,
    group_id: String,
}

pub async fn list(State(state): State<AppState>) -> Result<Json<ListRes>, AppError> {
    let mut file = fs::File::open(state.examples.join("config.yaml")).await?;
    let mut contents = vec![];
    file.read_to_end(&mut contents).await?;
    let config = serde_yaml::from_slice(&contents)?;
    Ok(Json(config))
}
