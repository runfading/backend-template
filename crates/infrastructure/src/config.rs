use config::{Config, File};
use serde::Deserialize;
use std::sync::OnceLock;

#[derive(Debug, Clone, Deserialize)]
pub struct Settings {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    #[serde(default)]
    pub log: LogConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    #[serde(default = "default_request_timeout_seconds")]
    pub request_timeout_seconds: u64,
    #[serde(default = "default_max_body_size_mb")]
    pub max_body_size_mb: usize,
    #[serde(default = "default_cors_origins")]
    pub cors_origins: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LogConfig {
    #[serde(default = "default_log_path")]
    pub log_path: String,
    #[serde(default = "default_name_prefix")]
    pub name_prefix: String,
    #[serde(default)]
    pub file_log_enable: bool,
    #[serde(default = "default_timezone_offset")]
    pub timezone_offset: i32,
}

fn default_log_path() -> String {
    "logs".to_string()
}

fn default_name_prefix() -> String {
    "{{project-name}}.log".to_string()
}

fn default_timezone_offset() -> i32 {
    8
}

fn default_request_timeout_seconds() -> u64 {
    30
}

fn default_max_body_size_mb() -> usize {
    2
}

fn default_cors_origins() -> Vec<String> {
    vec!["http://localhost:3000".to_string()]
}

impl Default for LogConfig {
    fn default() -> Self {
        Self {
            log_path: default_log_path(),
            name_prefix: default_name_prefix(),
            file_log_enable: false,
            timezone_offset: default_timezone_offset(),
        }
    }
}

pub fn load_config() -> Result<Settings, config::ConfigError> {
    Config::builder()
        .add_source(File::with_name("config/default"))
        .add_source(File::with_name("config/local").required(false))
        .build()?
        .try_deserialize()
}

pub static SETTINGS: OnceLock<Settings> = OnceLock::new();

pub fn init_config() -> Result<(), config::ConfigError> {
    let settings = load_config()?;
    SETTINGS.set(settings).expect("配置初始化失败");
    Ok(())
}
