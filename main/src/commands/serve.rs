use std::sync::Arc;

use clap::{value_parser, Arg, ArgMatches, Command};
use sea_orm::Database;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing::info;

use crate::{setting::Setting, state::AppState};

pub fn configure() -> Command {
    Command::new("serve").about("Start HTTP server").arg(
        Arg::new("port")
            .short('p')
            .long("port")
            .value_name("PORT")
            .help("TCP port to listen on")
            .default_value("8080")
            .value_parser(value_parser!(u16)),
    )
}

pub fn handle(matches: &ArgMatches, setting: &Setting) -> anyhow::Result<()> {
    if let Some(m) = matches.subcommand_matches("serve") {
        let port: u16 = *m.get_one("port").unwrap_or(&8080);

        start_tokio(port, setting)?;
    }

    Ok(())
}

fn start_tokio(port: u16, setting: &Setting) -> anyhow::Result<()> {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async move {
            let db_url = setting.database.url.clone().unwrap_or("".to_string());
            let db_conn = Database::connect(db_url)
                .await
                .expect("Database connection failed");

            let state = Arc::new(AppState::new(setting, db_conn)?);

            let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
            let router = crate::api::configure(state).layer(TraceLayer::new_for_http());

            info!("Starting axum on port {}", port);

            axum::serve(listener, router).await?;

            Ok::<(), anyhow::Error>(())
        })?;
    std::process::exit(0);
}
