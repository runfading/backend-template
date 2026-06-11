use config::{Config, Environment, File};
use serde::Deserialize;
use serde::de::DeserializeOwned;
use std::sync::OnceLock;
use tracing::info;

pub fn build<T>(name: &str) -> Result<T, config::ConfigError>
where
    T: DeserializeOwned,
{
    // 创建配置构建器
    let settings = Config::builder()
        // 从 example.toml 文件读取
        .add_source(config::File::with_name(name))
        .build()?;

    // 将配置解析为结构体
    let config = settings.try_deserialize()?;

    Ok(config)
}

#[derive(Debug, Clone, Deserialize)]
pub struct Settings {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
}

pub fn load_config() -> Result<Settings, config::ConfigError> {
    Ok(Config::builder()
        .add_source(File::with_name("config/default"))
        .add_source(File::with_name("config/local").required(false))
        .add_source(Environment::with_prefix("APP"))
        .build()?
        .try_deserialize()?)
}

pub static SETTINGS: OnceLock<Settings> = OnceLock::new();

pub fn init_config() -> Result<(), config::ConfigError> {
    let settings = load_config()?;
    SETTINGS.set(settings).expect("配置初始化失败");
    info!("配置初始化完成");
    Ok(())
}
