use thiserror::Error;

#[derive(Error, Debug)]
pub enum UserDomainError {
    #[error("用户名称重复")]
    UsernameAlreadyExists,

    #[error("电子邮箱格式错误")]
    EmailInvalid,
    #[error("电子邮箱必须填写")]
    EmailEmpty,
    #[error("电子邮箱重复")]
    EmailAlreadyExists,

    #[error("电话格式错误")]
    MobileInvalid,
    #[error("电话必须填写")]
    MobileEmpty,
    #[error("移动电话重复")]
    MobileAlreadyExists,

    #[error("用户账号暂停使用")]
    UserSuspended,
    #[error("用户不存在")]
    UserNotFound,

    #[error("系统内置资源不能修改或删除")]
    SystemResourceProtected,
    #[error("字段格式错误: {0}")]
    InvalidFields(String),
}
