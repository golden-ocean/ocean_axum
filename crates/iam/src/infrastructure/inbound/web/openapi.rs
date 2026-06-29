use utoipa::OpenApi;

use shared::prelude::Res;

use crate::infrastructure::inbound::web::{
    http_error::HttpError,
    user_handler,
    user_handler::{CreateUserReq, CreateUserRes, UpdateUserReq, UpdateUserRes, UserPageRes},
};

#[derive(OpenApi)]
#[openapi(
    paths(
        user_handler::create_user,
        user_handler::update_user,
        user_handler::delete_user,
        user_handler::get_user_page,
    ),
    components(
        schemas(
            CreateUserReq,
            CreateUserRes,
            UpdateUserReq,
            UpdateUserRes,
            UserPageRes,
            HttpError,
            Res<CreateUserRes>,
        )
    ),
    tags(
        (name = "Iam/User", description = "用户管理")
    )
)]
pub struct IamApiDoc;
