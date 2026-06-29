use std::sync::LazyLock;

use regex::Regex;

use crate::domain::error::UserDomainError;

static EMAIL_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+$").unwrap());

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Email(String);

impl Email {
    pub fn new(email: impl Into<String>) -> Result<Self, UserDomainError> {
        let email_str = email.into();

        if email_str.trim().is_empty() {
            return Err(UserDomainError::EmailEmpty);
        }

        if !EMAIL_REGEX.is_match(&email_str) {
            return Err(UserDomainError::EmailInvalid);
        }

        Ok(Self(email_str))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn from_storage(raw: String) -> Result<Self, UserDomainError> {
        if raw.is_empty() {
            return Err(UserDomainError::EmailInvalid);
        }
        Ok(Self(raw))
    }
}
