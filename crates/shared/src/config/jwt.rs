// --- JWT 配置 ---
#[derive(Clone, Default, serde::Deserialize)]
pub struct JwtConfig {
    #[serde(default = "default_secret")]
    pub secret: String,
    #[serde(default = "default_issuer")]
    pub issuer: String,
    #[serde(default = "default_access_token_exp")]
    pub access_token_exp_secs: u64, // 短凭证生存周期（如：2小时）
    #[serde(default = "default_refresh_token_exp")]
    pub refresh_token_exp_secs: u64, // 长凭证无感刷新周期（如：7天）
}

// 保证 Debug 时呈现打码状态
impl std::fmt::Debug for JwtConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JwtConfig")
            .field("secret", &"[REDACTED_FOR_SECURITY]") // 脱敏
            .field("access_token_exp_secs", &self.access_token_exp_secs)
            .field("refresh_token_exp_secs", &self.refresh_token_exp_secs)
            .finish()
    }
}
fn default_secret() -> String {
    "a-string-secret-at-least-256-bits-long".to_string()
}
fn default_issuer() -> String {
    "ocean-sys".to_string()
}
fn default_access_token_exp() -> u64 {
    7200
} // 2小时
fn default_refresh_token_exp() -> u64 {
    604800
} // 7天
