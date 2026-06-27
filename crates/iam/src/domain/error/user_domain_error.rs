use thiserror::Error;

#[derive(Error, Debug)]
pub enum UserDomainError {
    #[error("IAM_USER_USERNAME_ALREADY_EXISTS")]
    UsernameAlreadyExists,

    #[error("IAM_USER_EMAIL_INVALID_FORMAT")]
    EmailInvalid,
    #[error("IAM_USER_EMAIL_CANNOT_BE_EMPTY")]
    EmailEmpty,
    #[error("IAM_USER_EMAIL_ALREADY_EXISTS")]
    EmailAlreadyExists,

    #[error("IAM_USER_MOBILE_INVALID_FORMAT")]
    MobileInvalid,
    #[error("IAM_USER_MOBILE_CANNOT_BE_EMPTY")]
    MobileEmpty,
    #[error("IAM_USER_MOBILE_ALREADY_EXISTS")]
    MobileAlreadyExists,

    #[error("IAM_USER_STAFF_NO_INVALID_FORMAT")]
    StaffNoInvalid,
    #[error("IAM_USER_STAFF_NO_CANNOT_BE_EMPTY")]
    StaffNoEmpty,
    #[error("IAM_USER_STAFF_NO_ALREADY_EXISTS")]
    StaffNoAlreadyExists,

    #[error("IAM_USER_ACCOUNT_SUSPENDED")]
    UserSuspended,
    #[error("IAM_USER_USER_NOT_FOUND")]
    UserNotFound,

    #[error("IAM_SUSER_YSTEM_RESOURCE_PROTECTED")]
    SystemResourceProtected,
    #[error("IAM_USER_INVALID_FIELDS: {0}")]
    InvalidFields(String),
}
