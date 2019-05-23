use std::env;
use std::fs::File;
use std::io::prelude::*;

use toml;
use failure::Error;

#[derive(Debug, Deserialize, Clone)]
pub struct Configuration {
    pub api_key: String,
    pub city_id: String,
    pub units: String,
    pub display_symbol: String,
}

pub fn get_config() -> Result<Configuration, Error> {
    let mut config = env::home_dir().unwrap().to_str().unwrap().to_owned();
    config.push_str("/.config/polybar-forecast/config.toml");

    let mut f = File::open(config)?;
    let mut content = String::new();
    f.read_to_string(&mut content)?;

    let decoded: Configuration = toml::from_str(&content)?;

    Ok(decoded)
}
