use axum::http::HeaderValue;
use infrastructure::config::Settings;
use rest::RouterConfig;
use std::time::Duration;

pub fn init_router_config(setting: &Settings) -> RouterConfig {
    RouterConfig {
        request_timeout: Duration::from_secs(setting.server.request_timeout_seconds),
        max_body_size: setting
            .server
            .max_body_size_mb
            .checked_mul(1024 * 1024)
            .expect("max body size overflow"),
        cors_origins: setting
            .server
            .cors_origins
            .iter()
            .map(|origin| {
                origin
                    .parse::<HeaderValue>()
                    .unwrap_or_else(|_| panic!("invalid CORS origin: {origin}"))
            })
            .collect(),
    }
}
