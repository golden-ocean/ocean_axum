pub mod user_handler;

use axum::{
    Router,
    routing::{delete, get, post, put},
};
use sqlx::PgPool;
use std::sync::Arc;

use crate::{
    domain::repository::UserRepository, infrastructure::persistence::PostgresUserRepository,
};

#[derive(Clone)]
pub struct IamQueryState {
    pub reader_pool: PgPool,
}

#[derive(Clone)]
pub struct IamCommandState {
    pub user_repo: Arc<dyn UserRepository>,
}

pub fn init_iam_query_state(pool: PgPool) -> IamQueryState {
    IamQueryState {
        reader_pool: pool.clone(),
    }
}

pub fn init_iam_command_state(pool: PgPool) -> IamCommandState {
    IamCommandState {
        user_repo: Arc::new(PostgresUserRepository::new(pool.clone())),
    }
}

/// 模块内部路由表
pub fn iam_router(pool: PgPool) -> Router {
    let query_state = IamQueryState {
        reader_pool: pool.clone(),
    };
    let command_state = IamCommandState {
        user_repo: Arc::new(PostgresUserRepository::new(pool)),
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
