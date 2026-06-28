use shared::error::AppError;
use strum::{EnumDiscriminants, EnumString};
use thiserror::Error;

#[derive(Debug, Error, EnumDiscriminants)]
#[strum_discriminants(name(UserRepoErrorCode))]
#[strum_discriminants(derive(EnumString, strum::Display))]
#[strum_discriminants(strum(serialize_all = "SCREAMING_SNAKE_CASE"))]
pub enum UserRepoError {
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
}

impl UserRepoError {
    pub fn code(&self) -> String {
        format!("IAM_USER_REPO_{}", UserRepoErrorCode::from(self))
    }
}

impl From<UserRepoError> for AppError {
    fn from(e: UserRepoError) -> Self {
        let code = e.code();
        let msg = e.to_string();
        match UserRepoErrorCode::from(&e) {
            UserRepoErrorCode::NotFound => AppError::not_found(code, msg),

            UserRepoErrorCode::UsernameConflict
            | UserRepoErrorCode::EmailConflict
            | UserRepoErrorCode::MobileConflict => AppError::conflict(code, msg),

            UserRepoErrorCode::DatabaseError | UserRepoErrorCode::Unexpected => {
                AppError::InternalError(anyhow::anyhow!(e))
            }
            _ => AppError::bad_request(code, msg),
        }
    }
}
