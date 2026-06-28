use strum::{EnumDiscriminants, EnumString};
use thiserror::Error;

use shared::error::AppError;

#[derive(Debug, Error, EnumDiscriminants)]
#[strum_discriminants(name(UserDomainErrorCode))]
#[strum_discriminants(derive(EnumString, strum::Display))]
#[strum_discriminants(strum(serialize_all = "SCREAMING_SNAKE_CASE"))]
pub enum UserDomainError {
    #[error("用户名已存在")]
    UsernameAlreadyExists,

    #[error("邮箱格式无效")]
    EmailInvalid,
    #[error("邮箱不能为空")]
    EmailEmpty,
    #[error("邮箱已存在")]
    EmailAlreadyExists,

    #[error("手机号格式无效")]
    MobileInvalid,
    #[error("手机号不能为空")]
    MobileEmpty,
    #[error("手机号已存在")]
    MobileAlreadyExists,

    #[error("工号格式无效")]
    StaffNoInvalid,
    #[error("工号不能为空")]
    StaffNoEmpty,
    #[error("工号已存在")]
    StaffNoAlreadyExists,

    #[error("账户已被停用")]
    UserSuspended,
    #[error("用户不存在")]
    UserNotFound,

    #[error("系统内置资源受保护")]
    SystemResourceProtected,
    #[error("字段校验失败: {0}")]
    InvalidFields(String),
}

impl UserDomainError {
    pub fn code(&self) -> String {
        format!("IAM_USER_DOMAIN_{}", UserDomainErrorCode::from(self))
    }
}

impl From<UserDomainError> for AppError {
    fn from(e: UserDomainError) -> Self {
        let code = e.code();
        let msg = e.to_string();
        match UserDomainErrorCode::from(&e) {
            UserDomainErrorCode::UserNotFound => AppError::not_found(code, msg),

            UserDomainErrorCode::UsernameAlreadyExists
            | UserDomainErrorCode::EmailAlreadyExists
            | UserDomainErrorCode::MobileAlreadyExists
            | UserDomainErrorCode::StaffNoAlreadyExists => AppError::conflict(code, msg),

            UserDomainErrorCode::EmailEmpty
            | UserDomainErrorCode::MobileEmpty
            | UserDomainErrorCode::StaffNoEmpty
            | UserDomainErrorCode::EmailInvalid
            | UserDomainErrorCode::MobileInvalid
            | UserDomainErrorCode::StaffNoInvalid => AppError::bad_request(code, msg),

            _ => AppError::bad_request(code, msg),
        }
    }
}
