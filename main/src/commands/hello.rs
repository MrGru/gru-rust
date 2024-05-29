use clap::{ArgMatches, Command};

use crate::setting::Setting;

pub fn configure() -> Command {
    Command::new("hello").about("Hello world!")
}

pub fn handle(matches: &ArgMatches, _setting: &Setting) -> anyhow::Result<()> {
    if let Some(_m) = matches.subcommand_matches("hello") {
        println!("Hello world");
    }

    Ok(())
}
