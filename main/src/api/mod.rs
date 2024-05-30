use std::sync::Arc;

use axum::Router;

use crate::state::AppState;

mod error;
mod handlers;
mod json;
mod middleware;
mod request;
mod response;
mod v1;

pub fn configure(state: Arc<AppState>) -> Router {
    Router::new().nest("/v1", v1::configure(state))
}
