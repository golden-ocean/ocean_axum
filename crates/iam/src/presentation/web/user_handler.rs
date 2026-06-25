use axum::{Json, extract::State};
use serde::{Deserialize, Serialize};
use shared::crypto;
use shared::prelude::{AppError, Uuid};

use crate::application::commands::user::{CreateUserCommand, handle_create_user};
use crate::presentation::web::IamSliceState;

/// 前端网络请求载荷 (Request DTO)
#[derive(Debug, Deserialize)]
pub struct CreateUserReq {
    pub username: String,
    pub password: String,
    pub emp_no: String,
    pub name: String,
    pub email: String,
    pub mobile: String,
    pub organization_id: Option<Uuid>,
}

/// 后端网络响应载荷 (Response DTO)
#[derive(Debug, Serialize)]
pub struct CreateUserRes {
    pub user_id: Uuid,
}

/// 核心 Web 控制器
pub async fn create_user(
    State(state): State<IamSliceState>,
    // 假设系统中有 AuthExtractor 中间件，能全自动从 JWT 中捞出当前登录的操作人审计 ID
    // info_extractor: Extension<CurrentOperator>,
    Json(req): Json<CreateUserReq>,
) -> Result<Json<CreateUserRes>, AppError> {
    let (computed_hash, generated_salt) = crypto::hash_password(&req.password)
        .map_err(|_| AppError::InternalError(anyhow::anyhow!("CRYPTO_ENGINE_FAULT")))?;

    let current_operator_id = Some(Uuid::now_v7());

    let command = CreateUserCommand {
        username: req.username,
        password_hash: computed_hash,
        salt: generated_salt,
        emp_no: req.emp_no,
        name: req.name,
        email: req.email,
        mobile: req.mobile,
        organization_id: req.organization_id,
        operator_id: current_operator_id,
    };

    let new_user_id = handle_create_user(&*state.user_repo, command)
        .await
        .map_err(AppError::from)?;

    Ok(Json(CreateUserRes {
        user_id: new_user_id,
    }))
}
