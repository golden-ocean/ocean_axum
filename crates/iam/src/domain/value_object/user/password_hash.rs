use crate::domain::error::UserDomainError;

/// 密码哈希值对象
#[derive(Clone, PartialEq, Eq)]
pub struct PasswordHash(String);

impl PasswordHash {
    pub fn new(hash: String) -> Self {
        Self(hash)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }

    pub fn from_storage(raw: String) -> Result<Self, UserDomainError> {
        if !raw.starts_with('$') || raw.matches('$').count() < 3 {
            return Err(UserDomainError::PasswordHashInvalid);
        }
        Ok(Self(raw))
    }
}

/// 🔒 Debug 输出脱敏，防止日志泄露哈希值
impl std::fmt::Debug for PasswordHash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("PasswordHash").field(&"[REDACTED]").finish()
    }
}
