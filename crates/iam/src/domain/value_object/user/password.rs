use crate::domain::error::UserDomainError;

#[derive(Clone)]
pub struct Password(zeroize::Zeroizing<String>);

impl Password {
    pub fn new(raw: String) -> Result<Self, UserDomainError> {
        if raw.trim().is_empty() {
            return Err(UserDomainError::PasswordEmpty);
        }
        if raw.len() < 8 {
            return Err(UserDomainError::PasswordTooShort);
        }
        if raw.len() > 128 {
            return Err(UserDomainError::PasswordTooLong);
        }

        Ok(Self(zeroize::Zeroizing::new(raw)))
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }

    pub fn into_inner(self) -> zeroize::Zeroizing<String> {
        self.0
    }
}

impl std::fmt::Debug for Password {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Password").field(&"[REDACTED]").finish()
    }
}
