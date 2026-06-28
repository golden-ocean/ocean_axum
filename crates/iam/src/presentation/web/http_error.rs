use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use shared::prelude::{AppError, Res};

#[derive(Debug, utoipa::ToSchema)]
pub struct HttpError(pub AppError);

impl<E> From<E> for HttpError
where
    E: Into<AppError>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

// 统一对外网络报文协议映射
impl IntoResponse for HttpError {
    fn into_response(self) -> Response {
        let app_err = self.0;
        let (status, code, message) = match &app_err {
            AppError::BadRequest(c, m) => (StatusCode::BAD_REQUEST, c.clone(), m.clone()),
            AppError::NotFound(c, m) => (StatusCode::NOT_FOUND, c.clone(), m.clone()),
            AppError::Forbidden(c, m) => (StatusCode::FORBIDDEN, c.clone(), m.clone()),
            AppError::Unauthorized(c, m) => (StatusCode::UNAUTHORIZED, c.clone(), m.clone()),
            AppError::Conflict(c, m) => (StatusCode::CONFLICT, c.clone(), m.clone()),
            AppError::InternalError(e) => {
                tracing::error!("【致命系统底层故障触发】: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "INTERNAL_SERVER_ERROR".to_string(),
                    "服务器内部异常，请联系管理员".to_string(),
                )
            }
        };

        let body = Res::<()>::err(&code, &message);
        (status, Json(body)).into_response()
    }
}
