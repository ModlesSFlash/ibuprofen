use once_cell::sync::Lazy;
use std::{fs, path::Path};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct APP_CONFIG {
    pub google_api_key: String,
    pub google_cx: String,
    pub tenor_api_key: String,
    pub resolve_host: String,
    pub resolve_port: u16,
    pub download_dir: String,
}

pub static CONFIG: Lazy<APP_CONFIG> = Lazy::new(|| {
    let config_path = Path::new("config.toml");
    let config_str = fs::read_to_string(config_path)
        .expect("Failed to read config.toml");
    toml::from_str(&config_str).expect("Invalid config.toml format")
});
