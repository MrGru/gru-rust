use clap::{ArgMatches, Command};

use migration::{Migrator, MigratorTrait};
use sea_orm::Database;

use crate::setting::Setting;

pub fn configure() -> Command {
    Command::new("migrate").about("Run database migrations")
}

pub fn handle(matches: &ArgMatches, setting: &Setting) -> anyhow::Result<()> {
    if let Some(_m) = matches.subcommand_matches("migrate") {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async move {
                let db_url = setting.database.url.clone().unwrap_or("".to_string());
                let conn = Database::connect(db_url)
                    .await
                    .expect("Database connection failed");
                Migrator::up(&conn, None).await.unwrap();
            });
    }

    Ok(())
}
