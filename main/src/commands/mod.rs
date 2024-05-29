use clap::{ArgMatches, Command};

use crate::setting::Setting;

mod create_admin;
mod hello;
mod migrate;
mod serve;

pub fn configure(command: Command) -> Command {
    command
        .subcommand(hello::configure())
        .subcommand(serve::configure())
        .subcommand(migrate::configure())
        .subcommand(create_admin::configure())
}

pub fn handle(matches: &ArgMatches, setting: &Setting) -> anyhow::Result<()> {
    hello::handle(matches, setting)?;
    serve::handle(matches, setting)?;
    migrate::handle(matches, setting)?;
    create_admin::handle(matches, setting)?;

    Ok(())
}
