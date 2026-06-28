use axum::{
    Json,
    extract::{Path, Query, State},
};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

use shared::crypto;
use shared::prelude::{AppError, Page, Pagination, Res, Uuid};

use crate::{
    application::{
        commands::user::{
            CreateUserCommand, SoftDeleteUserCommand, UpdateUserCommand, handle_create_user,
            handle_soft_delete_user, handle_update_user,
        },
        queries::user::{UserPageQuery, handle_get_user_page},
    },
    presentation::web::{
        http_error::HttpError,
        router::{IamCommandState, IamQueryState},
    },
};

// =========================================================================
// Read 用户多条件可选分页查询 (Get User Page)
// =========================================================================
#[derive(Debug, Deserialize, Validate, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct GetUserPageReq {
    #[validate(range(min = 1, message = "页码必须从 1 开始"))]
    #[param(example = "1")]
    pub page: Option<u64>,
    #[validate(range(min = 1, max = 100, message = "每页条数必须在 1-100 之间"))]
    #[param(example = "10")]
    pub page_size: Option<u64>,

    #[param(example = "user")]
    pub username: Option<String>,
    #[param(example = "user@example.com")]
    pub email: Option<String>,
    #[param(example = "enabled")]
    pub status: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct UserPageRes {
    #[schema(example = "018f3d61-9c12-7bb3-a00d-5a81e9f1a234")]
    pub id: String,
    #[schema(example = "admin_user")]
    pub username: String,
    #[schema(example = "张三")]
    pub name: String,
    #[schema(example = "admin@example.com")]
    pub email: String,
    #[schema(example = "13800138000")]
    pub mobile: String,
    #[schema(example = "enabled")]
    pub status: String,
}

#[utoipa::path(
    get,
    path = "/users",
    params(GetUserPageReq),
    responses(
        (status = 200, description = "用户列表分页", body = Res<Page<UserPageRes>>),
        (status = 400, description = "请求参数校验失败", body = HttpError),
    ),
    tag = "IAM.User"
)]
pub async fn get_user_page(
    State(state): State<IamQueryState>,
    Query(req): Query<GetUserPageReq>,
) -> Result<Json<Res<Page<UserPageRes>>>, HttpError> {
    req.validate()
        .map_err(|e| AppError::bad_request("VALIDATION_ERROR", e.to_string()))?;

    let pagination = Pagination::new(req.page, req.page_size);

    let query = UserPageQuery {
        username: req.username,
        email: req.email,
        status: req.status,
        pagination,
    };

    let (records, total) = handle_get_user_page(&state.reader_pool, &query).await?;

    let res = Page::new(
        total as u64,
        pagination.page,
        pagination.page_size,
        records
            .into_iter()
            .map(|item| UserPageRes {
                id: item.id.to_string(),
                username: item.username,
                name: item.name,
                email: item.email,
                mobile: item.mobile,
                status: item.status.to_lowercase(),
            })
            .collect(),
    );
    Ok(Json(Res::ok(res)))
}

// =========================================================================
// Create 用户创建 (Create User)
// =========================================================================
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateUserReq {
    #[validate(length(min = 1, max = 50, message = "用户名长度必须在 1-50 之间"))]
    #[schema(example = "new_user")]
    pub username: String,
    #[validate(length(min = 6, max = 100, message = "密码长度必须在 6-100 之间"))]
    #[schema(example = "password123")]
    pub password: String,
    #[validate(length(min = 1, max = 50, message = "姓名长度必须在 1-50 之间"))]
    #[schema(example = "李四")]
    pub name: String,
    #[validate(email(message = "邮箱格式不正确"))]
    #[schema(example = "lisi@example.com")]
    pub email: String,
    #[validate(length(min = 11, max = 11, message = "手机号必须是 11 位"))]
    #[schema(example = "13912345678")]
    pub mobile: String,
    #[schema(example = "018f3d61-9c12-7bb3-a00d-5a81e9f1a234")]
    pub organization_id: Option<Uuid>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CreateUserRes {
    #[schema(example = "018f3d61-9c12-7bb3-a00d-5a81e9f1a235")]
    pub user_id: String,
}

#[utoipa::path(
    post,
    path = "/users",
    request_body = CreateUserReq,
    responses(
        (status = 200, description = "用户创建成功", body = Res<CreateUserRes>),
        (status = 400, description = "参数校验错误", body = HttpError),
        (status = 409, description = "用户名/邮箱已存在冲突", body = HttpError)
    ),
    tag = "IAM.User"
)]
pub async fn create_user(
    State(state): State<IamCommandState>,
    Json(req): Json<CreateUserReq>,
) -> Result<Json<Res<CreateUserRes>>, HttpError> {
    req.validate()
        .map_err(|e| AppError::bad_request("VALIDATION_ERROR", e.to_string()))?;

    let (computed_hash, generated_salt) = crypto::hash_password(&req.password)
        .map_err(|e| anyhow::anyhow!("Password hashing failed: {e}"))?;

    // TODO: 替换为真实的 AuthExtractor 提取当前操作人
    let current_operator_id = Some(Uuid::now_v7());

    let command = CreateUserCommand {
        username: req.username,
        password_hash: computed_hash,
        salt: generated_salt,
        name: req.name,
        email: req.email,
        mobile: req.mobile,
        organization_id: req.organization_id,
        operator_id: current_operator_id,
    };

    let new_user_id = handle_create_user(&*state.uow_manager, command).await?;

    Ok(Json(Res::ok(CreateUserRes {
        user_id: new_user_id.to_string(),
    })))
}

// =========================================================================
// Update 用户修改 (Update User)
// =========================================================================
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateUserReq {
    #[validate(length(min = 4, max = 30, message = "用户名长度必须在 4-30 位之间"))]
    #[schema(example = "update_username")]
    pub username: String,
    #[validate(length(min = 1, max = 50, message = "真实姓名不能为空且不能超过 50 位"))]
    #[schema(example = "王五")]
    pub name: String,
    #[validate(email(message = "传入的邮箱格式不正确"))]
    #[schema(example = "wangwu@example.com")]
    pub email: String,
    #[validate(length(min = 11, max = 11, message = "手机号必须为 11 位"))]
    #[schema(example = "13912345679")]
    pub mobile: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct UpdateUserRes {
    #[schema(example = "018f3d61-9c12-7bb3-a00d-5a81e9f1a235")]
    pub user_id: String,
}

#[utoipa::path(
    put,
    path = "/users/{id}",
    params(
        ("id" = Uuid, Path, description = "用户ID (UUIDv7)", example = "018f3d61-9c12-7bb3-a00d-5a81e9f1a234")
    ),
    request_body = UpdateUserReq,
    responses(
        (status = 200, description = "用户信息修改成功", body = Res<UpdateUserRes>),
        (status = 400, description = "参数校验错误", body = HttpError),
        (status = 404, description = "该用户不存在", body = HttpError),
        (status = 409, description = "用户名/邮箱已存在冲突", body = HttpError)
    ),
    tag = "IAM.User"
)]
pub async fn update_user(
    State(state): State<IamCommandState>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateUserReq>,
) -> Result<Json<Res<UpdateUserRes>>, HttpError> {
    req.validate()
        .map_err(|e| AppError::bad_request("VALIDATION_ERROR", e.to_string()))?;

    // TODO: 替换为真实的 AuthExtractor 提取当前操作人
    let current_operator_id = Uuid::nil();

    let command = UpdateUserCommand {
        id,
        username: req.username,
        name: req.name,
        email: req.email,
        mobile: req.mobile,
        operator_id: current_operator_id,
    };

    handle_update_user(&*state.uow_manager, command).await?;

    Ok(Json(Res::ok(UpdateUserRes {
        user_id: id.to_string(),
    })))
}

// =========================================================================
// Delete 用户删除 (Delete User)
// =========================================================================
#[derive(Debug, Serialize, ToSchema)]
pub struct DeleteUserRes {
    #[schema(example = "018f3d61-9c12-7bb3-a00d-5a81e9f1a235")]
    pub user_id: String,
}

#[utoipa::path(
    delete,
    path = "/users/{id}",
    params(
        ("id" = Uuid, Path, description = "删除的用户 ID", example = "018f3d61-9c12-7bb3-a00d-5a81e9f1a234")
    ),
    responses(
        (status = 200, description = "用户软删除成功", body = Res<DeleteUserRes>),
        (status = 404, description = "未找到该用户", body = HttpError)
    ),
    tag = "IAM.User"
)]
pub async fn delete_user(
    State(state): State<IamCommandState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Res<DeleteUserRes>>, HttpError> {
    // TODO: 替换为真实的 AuthExtractor 提取当前操作人
    let current_operator_id = Uuid::nil();

    let command = SoftDeleteUserCommand {
        id,
        operator_id: current_operator_id,
    };

    handle_soft_delete_user(&*state.uow_manager, command).await?;

    Ok(Json(Res::ok(DeleteUserRes {
        user_id: id.to_string(),
    })))
}
