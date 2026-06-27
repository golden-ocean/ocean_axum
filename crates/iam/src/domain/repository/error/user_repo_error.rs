#[derive(Debug, thiserror::Error)]
pub enum UserRepoError {
    #[error("IAM_USER_NOT_FOUND")]
    NotFound,
    // --- 用户特有的冲突字段平铺 ---
    #[error("IAM_USER_USERNAME_ALREADY_EXISTS")]
    UsernameConflict,
    #[error("IAM_USER_EMAIL_ALREADY_EXISTS")]
    EmailConflict,
    #[error("IAM_USER_MOBILE_ALREADY_EXISTS")]
    PhoneConflict,
    // --- 兜底与公共错误 ---
    #[error("IAM_USER_DATA_INCONSISTENT:{0}")]
    DataInconsistent(String),
    #[error("IAM_USER_CONCURRENCY_CONFLICT:{0}")]
    ConcurrencyConflict(String),
    #[error("IAM_USER_UNKNOWN_CONSTRAINT_VIOLATION:{0}")]
    UnknownConflict(String),
    #[error("IAM_USER_INTERNAL_SYSTEM_ERROR:{0}")]
    Unexpected(String),
    #[error("IAM_USER_DATABASE_ERROR:{0}")]
    DatabaseError(String),
}
