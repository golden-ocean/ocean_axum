use crate::application::error::UserAppError;
use shared::pagination::{PageRes, Pagination};
use sqlx::PgPool;

/// ⭐️ 组合用例入参：将多条件业务过滤参数与技术分页基件无缝融合
pub struct UserPageQuery {
    pub pagination: Pagination,
    pub username: Option<String>,
    pub email: Option<String>,
    pub status: Option<String>,
}

/// 核心层内部声明的出站扁平 DTO：没有打上序列化标记，纯洁且无防腐泄露
pub struct UserPageItemRes {
    pub id: String,
    pub username: String,
    pub name: String,
    pub email: String,
    pub mobile: String,
}

/// ⭐️ CQRS 直接写 SQL 派完全体：直插 Pool，强类型宏拦截，无 Trait 负累
pub async fn handle_get_user_page(
    pool: &PgPool,
    query: UserPageQuery,
) -> Result<PageRes<UserPageItemRes>, UserAppError> {
    // Step 1: 提取安全的限制值与偏移值算子
    let limit = query.pagination.limit();
    let offset = query.pagination.offset();

    // Step 2: 动态查总记录数
    // 利用 ($1::varchar IS NULL OR field = $1) 机制配合 UK 索引，彻底免除拼装字符串的工程隐患
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
    .map_err(|e| crate::domain::repository::error::UserRepoError::Unexpected(e.to_string()))?;

    // Step 3: 动态获取列表投影模型
    // 投喂 $4 (limit) 和 $5 (offset)，通过 PostgreSQL 内存索引树微秒级秒回
    let rows = sqlx::query!(
        r#"
        SELECT id, username, name, email, mobile
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
        limit,          // $4 -> 安全的 limit 算子
        offset          // $5 -> 安全的 offset 算子
    )
    .fetch_all(pool)
    .await
    .map_err(|e| crate::domain::repository::error::UserRepoError::Unexpected(e.to_string()))?;

    // Step 4: 扁平数据映射
    let res: Vec<UserPageItemRes> = rows
        .into_iter()
        .map(|r| UserPageItemRes {
            id: r.id.to_string(),
            username: r.username,
            name: r.name,
            email: r.email,
            mobile: r.mobile,
        })
        .collect();

    // Step 5: ⭐️ 完美组装你定义的 PageRes
    // 内部会自动流转 `(total + page_size - 1) / page_size` 闭环计算出 total_pages，毫无隐患
    Ok(PageRes::new(
        total as u64,
        query.pagination.page,
        query.pagination.page_size,
        res,
    ))
}
