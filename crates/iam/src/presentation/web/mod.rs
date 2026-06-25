pub mod user_handler;

use axum::{Router, routing::post};
use sqlx::PgPool;
use std::sync::Arc;

use crate::{
    domain::repository::UserRepository, infrastructure::persistence::PostgresUserRepository,
};

/// 模块上下文表现层共享状态
#[derive(Clone)]
pub struct IamSliceState {
    // pub user_repo: Arc<dyn UserRepository + Send + Sync>,
    pub user_repo: Arc<dyn UserRepository>,
}

/// 模块初始化
pub fn init_iam_slice(pool: PgPool) -> IamSliceState {
    let user_repo = PostgresUserRepository::new(pool);
    let user_repo_dyn = Arc::new(user_repo);

    IamSliceState {
        user_repo: user_repo_dyn,
    }
}

/// 模块内部路由表
pub fn iam_router() -> Router<IamSliceState> {
    Router::new().route("/users", post(user_handler::create_user))

    // 未来纵向扩展时无脑在这里平铺即可：
    // .route("/roles", post(role_handler::create_role))
    // .route("/permissions", post(perm_handler::create_perm))
}
