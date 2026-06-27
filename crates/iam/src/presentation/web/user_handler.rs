use axum::{
    Json,
    extract::{Path, Query, State},
};
use serde::{Deserialize, Serialize};
use validator::Validate;

use shared::crypto;
use shared::pagination::{PageRes, Pagination};
use shared::prelude::{AppError, Res, Uuid};

use crate::{
    application::commands::user::{
        CreateUserCommand, SoftDeleteUserCommand, UpdateUserCommand, handle_create_user,
        soft_delete_user::handle_soft_delete_user, update_user::handle_update_user,
    },
    presentation::web::IamQueryState,
};
use crate::{
    application::queries::user::get_user_page::{UserPageQuery, handle_get_user_page},
    presentation::web::IamCommandState,
};

// =========================================================================
// Read 用户多条件可选分页查询 (Get User Page)
// =========================================================================
#[derive(Debug, Deserialize, Validate)]
pub struct GetUserPageReq {
    #[validate(range(min = 1, message = "页码必须从 1 开始"))]
    pub page: Option<u64>,
    #[validate(range(min = 1, max = 100, message = "每页条数必须在 1-100 之间"))]
    pub page_size: Option<u64>,

    pub username: Option<String>,
    pub email: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct UserPageRes {
    pub id: String,
    pub username: String,
    pub name: String,
    pub email: String,
    pub mobile: String,
    pub status: String,
}

pub async fn get_user_page(
    State(state): State<IamQueryState>,
    Query(req): Query<GetUserPageReq>,
) -> Result<Json<PageRes<UserPageRes>>, AppError> {
    req.validate()
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    let pagination = Pagination::new(req.page, req.page_size);

    let query = UserPageQuery {
        username: req.username,
        email: req.email,
        status: req.status,
        pagination,
    };

    let (records, total) = handle_get_user_page(&state.reader_pool, &query)
        .await
        .map_err(AppError::from)?;

    let res = PageRes::new(
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

    Ok(Json(res))
}

// =========================================================================
// Create 用户创建 (Create User)
// =========================================================================
#[derive(Debug, Deserialize, Validate)]
pub struct CreateUserReq {
    #[validate(length(min = 1, max = 50, message = "用户名长度必须在 1-50 之间"))]
    pub username: String,
    #[validate(length(min = 6, max = 100, message = "密码长度必须在 6-100 之间"))]
    pub password: String,
    #[validate(length(min = 1, max = 50, message = "姓名长度必须在 1-50 之间"))]
    pub name: String,
    #[validate(email(message = "邮箱格式不正确"))]
    pub email: String,
    #[validate(length(min = 11, max = 11, message = "手机号必须是 11 位"))]
    pub mobile: String,
    pub organization_id: Option<Uuid>,
}

#[derive(Debug, Serialize)]
pub struct CreateUserRes {
    pub user_id: String,
}

pub async fn create_user(
    State(state): State<IamCommandState>,
    // 假设系统中有 AuthExtractor 中间件，能全自动从 JWT 中捞出当前登录的操作人审计 ID
    // info_extractor: Extension<CurrentOperator>,
    Json(req): Json<CreateUserReq>,
) -> Result<Json<CreateUserRes>, AppError> {
    req.validate()
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    let (computed_hash, generated_salt) = crypto::hash_password(&req.password)
        .map_err(|_| AppError::InternalError(anyhow::anyhow!("CRYPTO_ENGINE_FAULT")))?;

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

    let new_user_id = handle_create_user(&*state.uow_manager, command)
        .await
        .map_err(AppError::from)?;

    Ok(Json(CreateUserRes {
        user_id: new_user_id.to_string(),
    }))
}

// =========================================================================
// Update 用户修改 (Update User)
// =========================================================================
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateUserReq {
    #[validate(length(min = 4, max = 30, message = "用户名长度必须在 4-30 位之间"))]
    pub username: String,
    #[validate(length(min = 1, max = 50, message = "真实姓名不能为空且不能超过 50 位"))]
    pub name: String,
    #[validate(email(message = "传入的邮箱格式不正确"))]
    pub email: String,
    #[validate(length(min = 11, max = 11, message = "手机号必须为 11 位"))]
    pub mobile: String,
}

pub async fn update_user(
    State(state): State<IamCommandState>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateUserReq>,
) -> Result<Json<Res<()>>, AppError> {
    req.validate()
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    // todo: 从全局认证中间件提取安全审计操作员 ID
    let current_operator_id = Uuid::nil();

    let command = UpdateUserCommand {
        id,
        username: req.username,
        name: req.name,
        email: req.email,
        mobile: req.mobile,
        operator_id: current_operator_id,
    };

    handle_update_user(&*state.uow_manager, command)
        .await
        .map_err(AppError::from)?;

    Ok(Json(Res::ok(())))
}

// =========================================================================
// Delete 用户删除 (Delete User)
// =========================================================================
pub async fn delete_user(
    State(state): State<IamCommandState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Res<()>>, AppError> {
    // todo: 从全局认证中间件提取安全审计操作员 ID
    let current_operator_id = Uuid::nil();

    let command = SoftDeleteUserCommand {
        id,
        operator_id: current_operator_id,
    };

    handle_soft_delete_user(&*state.uow_manager, command)
        .await
        .map_err(AppError::from)?;

    Ok(Json(Res::ok(())))
}
