use clap::{Arg, Command};
use dotenv::dotenv;
use main::{commands, setting};
use tracing::{level_filters::LevelFilter, subscriber::set_global_default, Level};
use tracing_subscriber::{fmt::Layer, layer::SubscriberExt, Registry};

fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let mut command = Command::new("Test sample application")
        .version("1.0")
        .about("Boilerplate application in Rust")
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .help("Configuration file location")
                .default_value("config.json"),
        );
    command = commands::configure(command);

    let matches = command.get_matches();

    let config_location = matches
        .get_one::<String>("config")
        .map(|s| s.as_str())
        .unwrap_or("");

    let setting = setting::Setting::new(config_location, "GRUST")?;

    let subscriber = Registry::default()
        .with(LevelFilter::from_level(Level::DEBUG))
        .with(Layer::default().with_writer(std::io::stdout));

    set_global_default(subscriber).expect("Failed to set subscriber");

    commands::handle(&matches, &setting)?;

    Ok(())
}
