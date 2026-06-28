mod openapi;
mod router;
mod state;

use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;

use crate::router::create_router;
use crate::state::AppState;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 读取环境配置
    let app_config = shared::config::load_config()
        .expect("❌ 【环境初始化致命失败】系统配置物理参数不合规，拒绝进程上线，就地熔断保护");

    // 初始化日志
    shared::telemetry::logger::init_logger(&app_config.logger);
    tracing::info!(
        "⏰ 【环境配置加载成功】：统一配置加载就绪。当前所处运行环境级别: [{}]",
        app_config.server.env
    );

    // 初始化数据库连接池
    let db_pool = PgPoolOptions::new()
        .max_connections(app_config.database.max_connections)
        .min_connections(app_config.database.min_connections)
        .acquire_timeout(std::time::Duration::from_secs(
            app_config.database.acquire_timeout_secs,
        ))
        .connect(&app_config.database.url)
        .await
        .map_err(|e| {
            tracing::error!(
                "❌ 【数据库连接池初始化失败】PostgreSQL 连接池断开，拒绝响应: {}",
                e
            );
            e
        })?;

    tracing::info!(
        "📊 【数据库连接池初始化成功】：长连接桥梁连接池构建完毕。Max Active: {}",
        app_config.database.max_connections
    );

    // DI Assembling
    let global_state = AppState::new(db_pool.clone());

    // 初始化路由
    let router_total = create_router(global_state);

    let listen_address = format!("{}:{}", app_config.server.host, app_config.server.port);
    let listener = TcpListener::bind(&listen_address).await.map_err(|e| {
        tracing::error!(
            "❌ 【TCP 网络边缘监听端口被恶意抢占】监听端口 {} 被占用: {}",
            listen_address,
            e
        );
        e
    })?;

    tracing::info!("⭐ 【系统启动成功】监听端点开启: http://{}", listen_address);

    axum::serve(listener, router_total).await?;

    Ok(())
}
