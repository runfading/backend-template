use config::{Config, Environment, File};
use serde::Deserialize;
use std::sync::OnceLock;

#[derive(Debug, Clone, Deserialize)]
pub struct Settings {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    #[serde(default = "Default::default")]
    pub log: LogConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct LogConfig {
    pub log_path: String,
    pub name_prefix: String,
    pub file_log_enable: bool,
}

impl Default for LogConfig {
    fn default() -> Self {
        Self {
            log_path: "logs".to_string(),
            name_prefix: "{{project-name}}.log".to_string(),
            file_log_enable: false,
        }
    }
}

pub fn load_config() -> Result<Settings, config::ConfigError> {
    Config::builder()
        .add_source(File::with_name("config/default"))
        .add_source(File::with_name("config/local").required(false))
        .add_source(Environment::with_prefix("APP"))
        .build()?
        .try_deserialize()
}

pub static SETTINGS: OnceLock<Settings> = OnceLock::new();

pub fn init_config() -> Result<(), config::ConfigError> {
    let settings = load_config()?;
    SETTINGS.set(settings).expect("配置初始化失败");
    Ok(())
}
