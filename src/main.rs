mod db;
mod router;

use crate::db::init_db;
use crate::router::init_router_config;
use infrastructure::config::{LogConfig, SETTINGS, init_config};
{%- if auth %}
use rest::JwtConfig;
{%- endif %}
use rest::{AppState, init_router};
use tracing::info;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

#[tokio::main]
async fn main() {
    init_config().expect("配置初始化失败");
    let setting = SETTINGS.get().expect("配置有误");
    let _guard = init_logging(&setting.log);
    let pool = init_db(&setting.database).await.expect("数据库连接失败");

{%- if auth %}
    let state = AppState::new(
        pool,
        JwtConfig::new(
            setting.auth.jwt_secret.clone(),
            setting.auth.access_token_ttl_seconds,
        ),
    );
{%- else %}
    let state = AppState::new(pool);
{%- endif %}
    let router_config = init_router_config(setting);

    let listener =
        tokio::net::TcpListener::bind(format!("{}:{}", setting.server.host, setting.server.port))
            .await
            .expect("服务端口绑定失败");

    info!("服务启动成功");
    axum::serve(listener, init_router(state, router_config))
        .await
        .expect("服务启动失败");
}

fn init_logging(log_config: &LogConfig) -> Option<tracing_appender::non_blocking::WorkerGuard> {
    // 1. 设置日志文件存放目录和文件名前缀
    let directory = log_config.log_path.as_str();
    let file_name_prefix = log_config.name_prefix.as_str();

    // 自定义时间格式（可按需调整）
    let offset = time::UtcOffset::from_hms(log_config.timezone_offset as i8, 0, 0)
        .expect("invalid utc offset");
    let timer = tracing_subscriber::fmt::time::OffsetTime::new(
        offset,
        time::format_description::well_known::Rfc3339,
    );

    if log_config.file_log_enable {
        // 2. 配置每日滚动 (Daily Rotation)
        let file_appender = tracing_appender::rolling::daily(directory, file_name_prefix);

        // 3. 构造非阻塞写入器 (Non-blocking writer)
        // guard 必须在 main 中持有，如果它被丢弃，日志缓冲区的内容可能无法写入文件
        let (non_blocking_writer, guard) = tracing_appender::non_blocking(file_appender);

        // 4. 将文件输出与控制台输出结合
        tracing_subscriber::registry()
            // 过滤器：从环境变量 RUST_LOG 读取，默认为 info
            .with(
                tracing_subscriber::EnvFilter::try_from_default_env()
                    .unwrap_or_else(|_| "info".into()),
            )
            // 终端输出层
            .with(tracing_subscriber::fmt::layer().with_timer(timer.clone()))
            // 文件输出层（这里可以设置是否需要 ANSI 颜色）
            .with(
                tracing_subscriber::fmt::layer()
                    .with_ansi(false)
                    .with_writer(non_blocking_writer)
                    .with_timer(timer),
            )
            .init();
        Some(guard)
    } else {
        tracing_subscriber::registry()
            .with(
                tracing_subscriber::EnvFilter::try_from_default_env()
                    .unwrap_or_else(|_| "info".into()),
            )
            .with(tracing_subscriber::fmt::layer().with_timer(timer))
            .init();
        None
    }
}
