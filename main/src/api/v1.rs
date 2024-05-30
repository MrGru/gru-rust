use super::{handlers, middleware::jwt::auth};
use crate::state::AppState;
use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use std::sync::Arc;

pub fn configure(state: Arc<AppState>) -> Router {
    Router::new()
        .route(
            "/hello",
            get(handlers::hello::hello).with_state(state.clone()),
        )
        .route(
            "/login",
            post(handlers::login::login).with_state(state.clone()),
        )
        .route(
            "/items",
            post(handlers::item::create)
                .with_state(state.clone())
                .route_layer(middleware::from_fn_with_state(state.clone(), auth)),
        )
        .route(
            "/items",
            get(handlers::item::list).with_state(state.clone()),
        )
        .route("/items/:id", get(handlers::item::get).with_state(state))
}
