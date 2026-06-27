use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::sync::LazyLock;

use crate::domain::error::UserDomainError;

pub static STAFF_NO_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    // 定义工号的业务不变量格式（例如：STAFF-序号(6位)，如 STAFF-000001）
    Regex::new(r"^STAFF-\d{6}$").unwrap()
});

/// 必须满足特有的前缀及特定格式，且不可为空。
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct StaffNo(String);

impl StaffNo {
    pub fn new(staff_no: impl Into<String>) -> Result<Self, UserDomainError> {
        let staff_no_str = staff_no.into();

        if staff_no_str.trim().is_empty() {
            return Err(UserDomainError::StaffNoEmpty);
        }

        if !STAFF_NO_REGEX.is_match(&staff_no_str.trim()) {
            return Err(UserDomainError::StaffNoInvalid);
        }

        Ok(Self(staff_no_str))
    }

    pub fn reconstitute(value: String) -> Self {
        Self(value)
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl From<StaffNo> for String {
    fn from(staff_no: StaffNo) -> Self {
        staff_no.0
    }
}

impl fmt::Display for StaffNo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_staff_no() {
        let res = StaffNo::new("STAFF-000001");
        assert!(res.is_ok());
        assert_eq!(res.unwrap().value(), "STAFF-000001");
    }

    #[test]
    fn test_invalid_format() {
        let res = StaffNo::new("999999");
        assert!(res.is_err());

        let res_empty = StaffNo::new("   ");
        assert!(res_empty.is_err());
    }
}
