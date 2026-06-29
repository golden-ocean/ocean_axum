use thiserror::Error;

use shared::error::AppError;

use crate::application::ports::outbound::crypto::PasswordHashError;
use crate::application::ports::outbound::persistence::UserRepositoryError;
use crate::domain::error::UserDomainError;

/// IAM 应用服务层错误
#[derive(Debug, Error)]
pub enum IamAppError {
    #[error(transparent)]
    UserDomain(#[from] UserDomainError),
    #[error(transparent)]
    UserRepo(#[from] UserRepositoryError),

    // 🚀 未来扩展其他聚合根，对 HttpError 零破坏
    // #[error(transparent)]
    // RoleDomain(#[from] RoleDomainError),
    #[error("密码哈希失败: {0}")]
    PasswordHash(#[from] PasswordHashError),

    #[error("会话凭证已过期，请重新登录")]
    Unauthorized,
    #[error("无权执行当前操作")]
    Forbidden,
}

impl From<IamAppError> for AppError {
    fn from(err: IamAppError) -> Self {
        match err {
            IamAppError::UserDomain(e) => e.into(),
            IamAppError::UserRepo(e) => e.into(),

            // 未来新增聚合根只需加这一行，无需关心内部转换细节
            // IamAppError::RoleDomain(e) => e.into(),
            IamAppError::PasswordHash(e) => e.into(),
            IamAppError::Unauthorized => AppError::unauthorized(err.to_string()),
            IamAppError::Forbidden => AppError::forbidden(err.to_string()),
        }
    }
}
