#[derive(Debug, thiserror::Error, utoipa::ToSchema)]
pub enum AppError {
    #[error("{1}")] // Display 只显示 message，方便日志阅读
    BadRequest(String, String), // (code, message)

    #[error("{1}")]
    NotFound(String, String),

    #[error("{1}")]
    Forbidden(String, String),

    #[error("{1}")]
    Unauthorized(String, String),

    #[error("{1}")]
    Conflict(String, String),

    #[error(transparent)]
    #[schema(value_type = String)]
    InternalError(#[from] anyhow::Error),
}

impl AppError {
    pub fn bad_request(code: impl Into<String>, msg: impl Into<String>) -> Self {
        Self::BadRequest(code.into(), msg.into())
    }
    pub fn not_found(code: impl Into<String>, msg: impl Into<String>) -> Self {
        Self::NotFound(code.into(), msg.into())
    }
    pub fn conflict(code: impl Into<String>, msg: impl Into<String>) -> Self {
        Self::Conflict(code.into(), msg.into())
    }
    pub fn forbidden(msg: impl Into<String>) -> Self {
        Self::Forbidden("FORBIDDEN".into(), msg.into())
    }
    pub fn unauthorized(msg: impl Into<String>) -> Self {
        Self::Unauthorized("UNAUTHORIZED".into(), msg.into())
    }
}
