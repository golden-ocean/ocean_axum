// --- 日志配置 ---
#[derive(Debug, Clone, Default, serde::Deserialize)]
pub struct LoggerConfig {
    #[serde(default = "default_level")]
    pub level: String,
    #[serde(default = "default_format")]
    pub format: String, // "text" 供本地开发，"json" 供生产环境 ELK 日志收集
    #[serde(default = "default_path")]
    pub file_path: Option<String>,
}

// --- 默认参数 ---
fn default_level() -> String {
    "info".to_string()
}
fn default_format() -> String {
    "text".to_string()
}
fn default_path() -> Option<String> {
    Some("logs/app.log".to_string())
}
