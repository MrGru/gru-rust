use std::sync::Arc;

use axum::{extract::State, http::StatusCode};

use crate::state::AppState;

pub async fn hello(State(state): State<Arc<AppState>>) -> Result<String, StatusCode> {
    Ok(format!(
        "\nHello world! Using config from {} \n\n",
        state
            .setting
            .load()
            .config
            .location
            .clone()
            .unwrap_or("-".to_string())
    ))
}
