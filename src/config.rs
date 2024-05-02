use config::{Config, ConfigError, File};
use serde::Deserialize;

const CONFIG_FILEPATH: &str = "configs/default.yaml";

#[derive(Clone, Debug, Deserialize)]
pub struct Configuration {
    pub version: String,
    pub diagnostic_port: u16,
    pub communication_port: u16,
}

pub fn load_config() -> Result<Configuration, ConfigError> {
    let builder = Config::builder().add_source(File::with_name(CONFIG_FILEPATH));

    let config = builder.build()?;
    config.try_deserialize::<Configuration>()
}
