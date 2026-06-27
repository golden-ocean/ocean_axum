use crate::domain::error::UserDomainError;
use crate::domain::repository::error::UserRepoError;
use shared::prelude::AppError;

#[derive(Debug, thiserror::Error)]
pub enum UserAppError {
    #[error(transparent)]
    Domain(#[from] UserDomainError),

    #[error(transparent)]
    Repository(#[from] UserRepoError),

    #[error("会话凭证已过期，请重新登录")]
    Unauthorized,

    #[error("无权执行当前操作")]
    Forbidden,
}

impl From<UserAppError> for AppError {
    fn from(err: UserAppError) -> Self {
        match err {
            // UserDomainError -> AppError
            UserAppError::Domain(domain_err) => match domain_err {
                UserDomainError::UsernameAlreadyExists
                | UserDomainError::EmailAlreadyExists
                | UserDomainError::MobileAlreadyExists => {
                    AppError::Conflict(domain_err.to_string())
                }
                UserDomainError::UserNotFound => AppError::NotFound(domain_err.to_string()),

                UserDomainError::EmailInvalid
                | UserDomainError::EmailEmpty
                | UserDomainError::MobileInvalid
                | UserDomainError::MobileEmpty => AppError::BadRequest(domain_err.to_string()),

                UserDomainError::UserSuspended | UserDomainError::SystemResourceProtected => {
                    AppError::Forbidden(domain_err.to_string())
                }

                _ => AppError::BadRequest(domain_err.to_string()),
            },

            // UserRepoError -> AppError
            UserAppError::Repository(repo_err) => match repo_err {
                UserRepoError::UsernameConflict
                | UserRepoError::EmailConflict
                | UserRepoError::PhoneConflict => AppError::Conflict(repo_err.to_string()),

                UserRepoError::NotFound => AppError::NotFound(repo_err.to_string()),

                UserRepoError::ConcurrencyConflict(_) | UserRepoError::UnknownConflict(_) => {
                    AppError::Conflict(repo_err.to_string())
                }

                UserRepoError::DataInconsistent(e) | UserRepoError::Unexpected(e) => {
                    AppError::InternalError(anyhow::anyhow!(e))
                }
            },
            UserAppError::Unauthorized => {
                AppError::Unauthorized("IAM_AUTH_SESSION_EXPIRED".to_string())
            }
            UserAppError::Forbidden => {
                AppError::Forbidden("IAM_AUTH_PERMISSION_DENIED".to_string())
            }
        }
    }
}
