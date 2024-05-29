#![allow(unused)]

use std::os::unix::net::SocketAddr;

use axum::{routing::get, Router};

use crate::prelude::*;

mod error;
mod handlers;
mod prelude;
mod routes;
mod utils;

#[tokio::main]
fn main() -> Result<()> {
    let app = Router::new().route("/", get(routes::hello));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Server failed to start");

    Ok(())
}
