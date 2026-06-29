use shared::error::AppError;
use strum::{EnumDiscriminants, EnumString};
use thiserror::Error;

use crate::domain::error::UserDomainError;

#[derive(Debug, Error, EnumDiscriminants)]
#[strum_discriminants(name(UserRepositoryErrorCode))]
#[strum_discriminants(derive(EnumString, strum::Display))]
#[strum_discriminants(strum(serialize_all = "SCREAMING_SNAKE_CASE"))]
pub enum UserRepositoryError {
    #[error("用户记录未找到")]
    NotFound,

    #[error("用户名冲突")]
    UsernameConflict,
    #[error("邮箱冲突")]
    EmailConflict,
    #[error("手机号冲突")]
    MobileConflict,

    #[error("数据不一致: {0}")]
    DataInconsistent(String),
    #[error("并发冲突: {0}")]
    ConcurrencyConflict(String),
    #[error("未知约束违反: {0}")]
    UnknownConstraintViolation(String),

    #[error("非预期错误: {0}")]
    Unexpected(String),
    #[error("数据库错误: {0}")]
    DatabaseError(String),

    #[error("数据完整性损坏: {0}")]
    DataError(#[from] UserDomainError),
}

impl UserRepositoryError {
    pub fn code(&self) -> String {
        format!("IAM_USER_REPO_{}", UserRepositoryErrorCode::from(self))
    }
}

impl From<UserRepositoryError> for AppError {
    fn from(e: UserRepositoryError) -> Self {
        let code = e.code();
        let msg = e.to_string();
        match UserRepositoryErrorCode::from(&e) {
            UserRepositoryErrorCode::NotFound => AppError::not_found(code, msg),

            UserRepositoryErrorCode::UsernameConflict
            | UserRepositoryErrorCode::EmailConflict
            | UserRepositoryErrorCode::MobileConflict => AppError::conflict(code, msg),

            UserRepositoryErrorCode::DatabaseError | UserRepositoryErrorCode::Unexpected => {
                AppError::InternalError(anyhow::anyhow!(e))
            }
            _ => AppError::bad_request(code, msg),
        }
    }
}
