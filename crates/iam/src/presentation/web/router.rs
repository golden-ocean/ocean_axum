use std::sync::Arc;

use axum::{
    Router,
    routing::{delete, get, post, put},
};
use sqlx::PgPool;

use crate::{
    domain::repository::UnitOfWorkManager,
    infrastructure::persistence::postgres_uow::PostgresUnitOfWorkManager,
    presentation::web::user_handler,
};

#[derive(Clone)]
pub struct IamQueryState {
    pub reader_pool: PgPool,
}

#[derive(Clone)]
pub struct IamCommandState {
    pub uow_manager: Arc<dyn UnitOfWorkManager>,
}

/// 模块内部路由表
pub fn iam_router(pool: PgPool) -> Router {
    let query_state = IamQueryState {
        reader_pool: pool.clone(),
    };
    let command_state = IamCommandState {
        uow_manager: Arc::new(PostgresUnitOfWorkManager::new(pool)),
    };

    let query_routes = Router::new()
        .route("/users", get(user_handler::get_user_page))
        .with_state(query_state);

    let command_routes = Router::new()
        .route("/users", post(user_handler::create_user))
        .route("/users/{id}", put(user_handler::update_user))
        .route("/users/{id}", delete(user_handler::delete_user))
        .with_state(command_state);

    Router::new().merge(query_routes).merge(command_routes)

    // 未来纵向扩展时无脑在这里平铺即可：
    // .route("/roles", post(role_handler::create_role))
    // .route("/permissions", post(perm_handler::create_perm))
}
