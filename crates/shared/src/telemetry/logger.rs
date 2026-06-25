use crate::config::logger::LoggerConfig;
use std::str::FromStr;
use tracing::Level;

/// 全局唯一遥测日志生命周期
pub fn init_logger(config: &LoggerConfig) {
    // 安全解析日志级别，格式错误则为 INFO
    let log_level = Level::from_str(&config.level).unwrap_or(Level::INFO);

    // 日志格式
    if config.format == "json" {
        tracing_subscriber::fmt()
            .json()
            .with_max_level(log_level)
            .init();
    } else {
        tracing_subscriber::fmt().with_max_level(log_level).init();
    }
}
