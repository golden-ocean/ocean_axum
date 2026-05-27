#[derive(Debug, thiserror::Error)]
pub enum UserRepoError {
    #[error("用户不存在")]
    NotFound,

    // --- 用户特有的冲突字段平铺 ---
    #[error("用户名已被占用")]
    UsernameConflict,
    #[error("邮箱已被注册")]
    EmailConflict,
    #[error("手机号已被注册")]
    PhoneConflict,

    // --- 兜底与公共错误 ---
    #[error("数据库底层数据不一致/损坏: {0}")]
    DataInconsistent(String),
    #[error("未知字段冲突: {0}")]
    UnknownConflict(String),
    #[error("并发版本冲突")]
    ConcurrencyConflict(String),
    #[error("系统异常: {0}")]
    Unexpected(String),
}
