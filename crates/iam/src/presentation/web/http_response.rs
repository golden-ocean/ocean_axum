use axum::{
    Json,
    response::{IntoResponse, Response},
};
use serde::Serialize;

use shared::prelude::Res;

#[derive(Debug, utoipa::ToSchema)]
pub struct HttpResponse<T>(pub Res<T>);

impl<T> HttpResponse<T> {
    pub fn ok(data: T) -> Self
    where
        T: Serialize,
    {
        Self(Res::ok(data))
    }

    pub fn success() -> HttpResponse<()> {
        HttpResponse(Res::success())
    }
}

impl<T> IntoResponse for HttpResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        Json(self.0).into_response()
    }
}
