use sqlx::PgPool;

use shared::prelude::Pagination;

use crate::application::{error::IamAppError, ports::outbound::persistence::UserRepositoryError};

pub struct UserPageQuery {
    pub username: Option<String>,
    pub email: Option<String>,
    pub status: Option<String>,

    pub pagination: Pagination,
}

pub struct UserPageItem {
    pub id: shared::prelude::Uuid,
    pub username: String,
    pub name: String,
    pub email: String,
    pub mobile: String,
    pub status: String,
}

pub async fn handle_get_user_page(
    pool: &PgPool,
    query: &UserPageQuery,
) -> Result<(Vec<UserPageItem>, i64), IamAppError> {
    let limit = query.pagination.limit();
    let offset = query.pagination.offset();

    // 利用 ($1::varchar IS NULL OR field = $1) 机制配合 UK 索引，彻底免除拼装字符串的隐患
    let total = sqlx::query_scalar!(
        r#"
        SELECT COUNT(1) as "count!"
        FROM sys_user
        WHERE deleted_at IS NULL
          AND ($1::varchar IS NULL OR username LIKE '%' || $1 || '%')
          AND ($2::varchar IS NULL OR email = $2)
          AND ($3::varchar IS NULL OR status = $3)
        "#,
        query.username, // $1
        query.email,    // $2
        query.status    // $3
    )
    .fetch_one(pool)
    .await
    .map_err(|e| UserRepositoryError::Unexpected(e.to_string()))?;

    let rows = sqlx::query!(
        r#"
        SELECT id, username, name, email, mobile, status
        FROM sys_user
        WHERE deleted_at IS NULL
          AND ($1::varchar IS NULL OR username LIKE '%' || $1 || '%')
          AND ($2::varchar IS NULL OR email = $2)
          AND ($3::varchar IS NULL OR status = $3)
        ORDER BY created_at DESC
        LIMIT $4 OFFSET $5
        "#,
        query.username, // $1
        query.email,    // $2
        query.status,   // $3
        limit,          // $4
        offset          // $5
    )
    .fetch_all(pool)
    .await
    .map_err(|e| UserRepositoryError::Unexpected(e.to_string()))?;

    let res: Vec<UserPageItem> = rows
        .into_iter()
        .map(|r| UserPageItem {
            id: r.id,
            username: r.username,
            name: r.name,
            email: r.email,
            mobile: r.mobile,
            status: r.status,
        })
        .collect();

    Ok((res, total))
}
