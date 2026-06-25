// --- 数据库配置 ---
#[derive(Debug, Clone, Default, serde::Deserialize)]
pub struct DatabaseConfig {
    pub url: String, // 数据库连接串属于必填项，不提供默认值
    #[serde(default = "default_max_connections")]
    pub max_connections: u32,
    #[serde(default = "default_min_connections")]
    pub min_connections: u32,
    #[serde(default = "default_acquire_timeout_secs")]
    pub acquire_timeout_secs: u64, // 秒
}

fn default_max_connections() -> u32 {
    20
}
fn default_min_connections() -> u32 {
    5
}
fn default_acquire_timeout_secs() -> u64 {
    3
}
