#![allow(unused)]

use axum::{routing::get, Router};
use tokio::net::{unix::SocketAddr, TcpListener};

use crate::prelude::*;

mod error;
mod handlers;
mod prelude;
mod routes;
mod utils;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(routes::hello));

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
