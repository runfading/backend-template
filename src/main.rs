pub mod demo;
pub mod common;
pub mod config;
pub mod db;
mod error;
pub mod routers;

use crate::common::AppState;
use crate::config::{SETTINGS, init_config};
use crate::db::init_db;
use crate::routers::init_router;
use tracing::info;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

#[tokio::main]
async fn main() {
    init_log();
    init_config().expect("配置初始化失败");
    let setting = SETTINGS.get().expect("配置有误");
    let pool = init_db(&setting.database.url)
        .await
        .expect("数据库连接失败");

    let state = AppState { db: pool };

    let listener =
        tokio::net::TcpListener::bind(format!("{}:{}", setting.server.host, setting.server.port))
            .await
            .expect("服务端口绑定失败");

    info!("服务启动成功");
    axum::serve(listener, init_router(state))
        .await
        .expect("服务启动失败");
}

fn init_log() {
    // 自定义时间格式（可按需调整）
    let offset = time::UtcOffset::from_hms(8, 0, 0).expect("invalid utc offset");
    let timer = tracing_subscriber::fmt::time::OffsetTime::new(
        offset,
        time::format_description::well_known::Rfc3339,
    );

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()),
        )
        .with(tracing_subscriber::fmt::layer().with_timer(timer))
        .init();
}
