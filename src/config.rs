use config::{Config, ConfigError, File};
use serde::Deserialize;

const CONFIG_FILEPATH: &str = "configs/config.yaml";

#[derive(Clone, Debug, Deserialize)]
pub struct CoreConfiguration {
    pub max_number_of_drones: usize,
    pub maximum_tickrate: f64,
    pub tickrate_calculation_period_ms: u64,
    pub perimeter_filepath: String,
    pub cell_size: f64,
}

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct CommunicationConfiguration {
    pub port: u16,
}

#[derive(Clone, Debug, Deserialize)]
pub struct DiagnosticConfiguration {
    pub enabled: bool,
    pub port: u16,
    pub host: String,
    pub use_permissive_cors: bool,
    pub concurrent_requests: usize,
    pub buffer_size: usize,
    pub rate_limit: u64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Configuration {
    pub core: CoreConfiguration,
    pub diagnostic: DiagnosticConfiguration,
    pub communication: CommunicationConfiguration,
}

pub fn load_config() -> Result<Configuration, ConfigError> {
    let builder = Config::builder().add_source(File::with_name(CONFIG_FILEPATH));

    let config = builder.build()?;
    config.try_deserialize::<Configuration>()
}
