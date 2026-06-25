// --- 服务器配置 ---
#[derive(Debug, Clone, Default, serde::Deserialize)]
pub struct ServerConfig {
    #[serde(default = "default_host")]
    pub host: String,
    #[serde(default = "default_port")]
    pub port: u16,
    #[serde(default = "default_env")]
    pub env: String,
}

// --- 默认参数 ---
fn default_host() -> String {
    "127.0.0.1".to_string()
}
fn default_port() -> u16 {
    3000
}
fn default_env() -> String {
    "development".to_string()
}
