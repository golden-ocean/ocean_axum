use std::sync::LazyLock;

use regex::Regex;

use crate::domain::error::UserDomainError;

// 静态编译手机号正则
pub static MOBILE_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    // 简单的 E.164 国际格式或国内 11 位手机号正则示例
    // 你可以根据实际业务需求随时修改这个正则，比如国内限定 r"^1[3-9]\d{9}$"
    Regex::new(r"^\+?[1-9]\d{7,14}$").unwrap()
});

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Mobile(String);

impl Mobile {
    pub fn new(mobile: impl Into<String>) -> Result<Self, UserDomainError> {
        let mobile_str = mobile.into();

        if mobile_str.trim().is_empty() {
            return Err(UserDomainError::MobileEmpty);
        }

        if !MOBILE_REGEX.is_match(&mobile_str) {
            return Err(UserDomainError::MobileInvalid);
        }

        Ok(Self(mobile_str))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn from_storage(raw: String) -> Result<Self, UserDomainError> {
        if raw.len() > 32 {
            return Err(UserDomainError::MobileInvalid);
        }
        Ok(Self(raw))
    }
}
