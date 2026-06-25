pub mod database;
pub mod jwt;
pub mod logger;
pub mod server;

use crate::config::{
    database::DatabaseConfig, jwt::JwtConfig, logger::LoggerConfig, server::ServerConfig,
};
use std::sync::OnceLock;

/// 全局单例
static CONFIG: OnceLock<AppConfig> = OnceLock::new();

/// App配置
#[derive(Debug, Clone, serde::Deserialize)]
pub struct AppConfig {
    #[serde(default)]
    pub server: ServerConfig,
    #[serde(default)]
    pub database: DatabaseConfig,
    #[serde(default)]
    pub logger: LoggerConfig,
    #[serde(default)]
    pub jwt: JwtConfig,
}

impl AppConfig {
    pub fn global() -> &'static Self {
        CONFIG
            .get()
            .expect("【系统灾难】尝试在配置中心读取全局配置，请检查启动生命周期时序")
    }
}

pub fn load_config() -> Result<&'static AppConfig, config::ConfigError> {
    if let Some(cfg) = CONFIG.get() {
        return Ok(cfg);
    }

    // 运行环境，默认本地开发 development
    let run_env = std::env::var("RUN_ENV").unwrap_or_else(|_| "development".to_string());

    // 加载项目根目录下的 `.env` 环境文件
    let _ = dotenvy::dotenv();

    let settings = config::Config::builder()
        // 加载基础配置
        .add_source(config::File::with_name("configs/base").required(false))
        // 加载运行配置（dev/prod）
        .add_source(config::File::with_name(&format!("configs/{}", run_env)).required(false))
        // 加载环境变量配置
        // 规则：所有以 `APP__` 开头的变量全自动识别。如：APP__DATABASE__MAX_CONNECTIONS=50
        .add_source(config::Environment::with_prefix("APP").separator("__"))
        .build()?;

    let final_config: AppConfig = settings.try_deserialize()?;

    let _ = CONFIG.set(final_config);

    Ok(AppConfig::global())
}
