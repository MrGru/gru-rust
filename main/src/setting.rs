use anyhow::Ok;
use config::{Config, Environment, File};
use serde::Deserialize;

#[derive(Debug, Default, Deserialize, Clone)]
pub struct Database {
    pub url: Option<String>,
}

#[derive(Debug, Default, Deserialize, Clone)]
pub struct Logging {
    pub log_level: Option<String>,
}

#[derive(Debug, Default, Deserialize, Clone)]
pub struct ConfigInfo {
    pub location: Option<String>,
    pub env_prefix: Option<String>,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct Setting {
    #[serde(default)]
    pub config: ConfigInfo,
    #[serde(default)]
    pub database: Database,
    #[serde(default)]
    pub logging: Logging,
    #[serde(default)]
    pub token_secret: String,
    #[serde(default)]
    pub token_timeout_seconds: i64,
}

impl Setting {
    pub fn new(location: &str, env_prefix: &str) -> anyhow::Result<Self> {
        let s = Config::builder()
            .add_source(File::with_name(location))
            .add_source(
                Environment::with_prefix(env_prefix)
                    .separator("__")
                    .prefix_separator("__"),
            )
            .set_override("config.location", location)?
            .set_override("config.env_prefix", env_prefix)?
            .build()?;
        let setting = s.try_deserialize()?;
        Ok(setting)
    }
}
