use axum::http::StatusCode;
use thiserror::Error;

use crate::http::response::Res;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("{0}")]
    BadRequest(String),
    #[error("{0}")]
    NotFound(String),
    #[error("{0}")]
    Forbidden(String),
    #[error("{0}")]
    Unauthorized(String),
    #[error("{0}")]
    Conflict(String),
    #[error(transparent)]
    InternalError(#[from] anyhow::Error),
}

impl AppError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            Self::BadRequest(_) => StatusCode::BAD_REQUEST,
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::Forbidden(_) => StatusCode::FORBIDDEN,
            Self::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            Self::Conflict(_) => StatusCode::CONFLICT,
            Self::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    pub fn error_code(&self) -> &'static str {
        match self {
            Self::BadRequest(_) => "BAD_REQUEST",
            Self::NotFound(_) => "NOT_FOUND",
            Self::Forbidden(_) => "FORBIDDEN",
            Self::Unauthorized(_) => "UNAUTHORIZED",
            Self::Conflict(_) => "RESOURCE_CONFLICT",
            Self::InternalError(_) => "INTERNAL_SERVER_ERROR",
        }
    }
}

impl axum::response::IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let status = self.status_code();
        let code = self.error_code();

        // 隐藏 InternalError 中的敏感 SQL 或网络详情
        let message = match self {
            Self::InternalError(ref e) => {
                tracing::error!(target: "shared::error", "系统内部致命异常触发: {:?}", e);
                "服务器内部异常，请联系管理员".to_string()
            }
            // 其他普通的业务级校验规则，直接向用户展示明文
            _ => self.to_string(),
        };

        let response_body = Res::<()>::err(code, &message);
        (status, response_body).into_response()
    }
}
