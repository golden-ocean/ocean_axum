use async_trait::async_trait;
use sqlx::{Postgres, Transaction};
use std::collections::HashSet;

use shared::prelude::Uuid;

use crate::domain::entity::User;
use crate::domain::repository::UserRepository;
use crate::domain::repository::error::UserRepoError;
use crate::domain::value_object::common::{RoleId, UserId};
use crate::domain::value_object::user::{Email, Mobile, StaffNo};
use crate::infrastructure::persistence::mapper::UserMapper;
use crate::infrastructure::persistence::model::UserRow;

/// 基于 PostgreSQL 的用户数据仓储实现适配器
pub struct PostgresUserRepository<'a> {
    pub tx: &'a mut Transaction<'static, Postgres>,
}

impl<'a> PostgresUserRepository<'a> {
    pub fn new(tx: &'a mut Transaction<'static, Postgres>) -> Self {
        Self { tx }
    }

    fn map_sqlx_error(e: sqlx::Error) -> UserRepoError {
        if let sqlx::Error::Database(pool_err) = &e {
            if pool_err.is_unique_violation() {
                let constraint = pool_err.constraint().unwrap_or_default();
                return match constraint {
                    "uk_sys_user_username" => UserRepoError::UsernameConflict,
                    "uk_sys_user_email" => UserRepoError::EmailConflict,
                    "uk_sys_user_mobile" => UserRepoError::MobileConflict,
                    _ => UserRepoError::UnknownConstraintViolation(constraint.to_string()),
                };
            }
        }
        UserRepoError::Unexpected(e.to_string())
    }

    async fn assemble_user(&mut self, row: UserRow) -> Result<User, UserRepoError> {
        let user_id = row.id;

        // 从中间关系表获取该用户当前拥建立多对多的角色 ID 集合
        let db_roles: Vec<Uuid> =
            sqlx::query_scalar("SELECT role_id FROM sys_user_role WHERE user_id = $1")
                .bind(user_id)
                .fetch_all(&mut **self.tx)
                .await
                .map_err(Self::map_sqlx_error)?;

        let role_ids: Vec<RoleId> = db_roles.into_iter().map(RoleId::from).collect();
        let user = UserMapper::to_entity(row, role_ids)?;

        Ok(user)
    }
}

#[async_trait]
impl<'a> UserRepository for PostgresUserRepository<'a> {
    /// 统一保存行为 (Upsert - 整体替换并持久化聚合根不变量)
    async fn save(&mut self, user: &User) -> Result<(), UserRepoError> {
        let row = UserMapper::to_row(user);

        // 1. 全量 Upsert 写入主表 sys_user
        sqlx::query!(
            r#"
            INSERT INTO sys_user (
                id, username, staff_no, name, email, mobile, gender, birthday, avatar,
                password_hash, salt, password_updated_at, work_status, data_scope,
                is_builtin, sort, remark, status,
                organization_id, position_id,
                created_at, updated_at, created_by, updated_by, deleted_at, deleted_by
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9,
                $10, $11, $12, $13, $14, $15, $16, $17, $18,
                $19, $20, $21, $22, $23, $24, $25, $26
            )
            ON CONFLICT (id) DO UPDATE SET
                username = EXCLUDED.username,
                staff_no = EXCLUDED.staff_no,
                name = EXCLUDED.name,
                email = EXCLUDED.email,
                mobile = EXCLUDED.mobile,
                gender = EXCLUDED.gender,
                birthday = EXCLUDED.birthday,
                avatar = EXCLUDED.avatar,
                password_hash = EXCLUDED.password_hash,
                salt = EXCLUDED.salt,
                password_updated_at = EXCLUDED.password_updated_at,
                work_status = EXCLUDED.work_status,
                data_scope = EXCLUDED.data_scope,
                is_builtin = EXCLUDED.is_builtin,
                sort = EXCLUDED.sort,
                remark = EXCLUDED.remark,
                status = EXCLUDED.status,
                organization_id = EXCLUDED.organization_id,
                position_id = EXCLUDED.position_id,
                updated_at = EXCLUDED.updated_at,
                updated_by = EXCLUDED.updated_by,
                deleted_at = EXCLUDED.deleted_at,
                deleted_by = EXCLUDED.deleted_by
            "#,
            row.id,
            row.username,
            row.staff_no,
            row.name,
            row.email,
            row.mobile,
            row.gender,
            row.birthday,
            row.avatar,
            row.password_hash,
            row.salt,
            row.password_updated_at,
            row.work_status,
            row.data_scope,
            row.is_builtin,
            row.sort,
            row.remark,
            row.status,
            row.organization_id,
            row.position_id,
            row.created_at,
            row.updated_at,
            row.created_by,
            row.updated_by,
            row.deleted_at,
            row.deleted_by
        )
        .execute(&mut **self.tx)
        .await
        .map_err(Self::map_sqlx_error)?;

        // 2. 内存集合比对优化：获取数据库内的多对多中间表历史快照
        let pool_roles = sqlx::query!(
            "SELECT role_id FROM sys_user_role WHERE user_id = $1",
            row.id
        )
        .fetch_all(&mut **self.tx)
        .await
        .map_err(Self::map_sqlx_error)?;

        let pool_role_set: HashSet<Uuid> = pool_roles.into_iter().map(|r| r.role_id).collect();
        let current_role_set: HashSet<Uuid> = user.role_ids().iter().map(|r| r.value()).collect();

        // 严格比对对称差集，按实际差额精准操作
        let roles_to_delete: Vec<Uuid> = pool_role_set
            .difference(&current_role_set)
            .cloned()
            .collect();
        let roles_to_insert: Vec<Uuid> = current_role_set
            .difference(&pool_role_set)
            .cloned()
            .collect();

        // 3. 执行物理关系网变更刷新
        if !roles_to_delete.is_empty() {
            sqlx::query!(
                "DELETE FROM sys_user_role WHERE user_id = $1 AND role_id = ANY($2)",
                row.id,
                &roles_to_delete
            )
            .execute(&mut **self.tx) // 🌟 统一对齐：通过双星号穿透
            .await
            .map_err(Self::map_sqlx_error)?;
        }

        if !roles_to_insert.is_empty() {
            sqlx::query!(
                r#"
                INSERT INTO sys_user_role (user_id, role_id)
                SELECT $1, * FROM UNNEST($2::uuid[])
                "#,
                row.id,
                &roles_to_insert
            )
            .execute(&mut **self.tx) // 🌟 统一对齐：通过双星号穿透
            .await
            .map_err(Self::map_sqlx_error)?;
        }

        Ok(())
    }

    /// 级联物理删除 (连带删除 用户-角色 关系)
    async fn remove(&mut self, user_id: &UserId) -> Result<(), UserRepoError> {
        sqlx::query!(
            "DELETE FROM sys_user_role WHERE user_id = $1",
            user_id.value()
        )
        .execute(&mut **self.tx)
        .await
        .map_err(Self::map_sqlx_error)?;

        sqlx::query!("DELETE FROM sys_user WHERE id = $1", user_id.value())
            .execute(&mut **self.tx)
            .await
            .map_err(Self::map_sqlx_error)?;

        Ok(())
    }

    /// 通过 ID 查找用户
    async fn find_by_id(&mut self, user_id: &UserId) -> Result<Option<User>, UserRepoError> {
        let row_opt = sqlx::query_as!(
            UserRow,
            "SELECT * FROM sys_user WHERE id = $1 AND deleted_at IS NULL",
            user_id.value()
        )
        .fetch_optional(&mut **self.tx)
        .await
        .map_err(Self::map_sqlx_error)?;

        match row_opt {
            Some(row) => {
                let user = self.assemble_user(row).await?;
                Ok(Some(user))
            }
            None => Ok(None),
        }
    }

    /// 通过强类型 Username 值对象检索用户
    async fn find_by_username(&mut self, username: &str) -> Result<Option<User>, UserRepoError> {
        let row_opt = sqlx::query_as!(
            UserRow,
            "SELECT * FROM sys_user WHERE username = $1 AND deleted_at IS NULL",
            username
        )
        .fetch_optional(&mut **self.tx)
        .await
        .map_err(Self::map_sqlx_error)?;

        match row_opt {
            Some(row) => {
                let user = self.assemble_user(row).await?;
                Ok(Some(user))
            }
            None => Ok(None),
        }
    }

    /// 通过手机号检索用户
    async fn find_by_mobile(&mut self, mobile: &Mobile) -> Result<Option<User>, UserRepoError> {
        let row_opt = sqlx::query_as!(
            UserRow,
            "SELECT * FROM sys_user WHERE mobile = $1 AND deleted_at IS NULL",
            mobile.as_str()
        )
        .fetch_optional(&mut **self.tx)
        .await
        .map_err(Self::map_sqlx_error)?;

        match row_opt {
            Some(row) => {
                let user = self.assemble_user(row).await?;
                Ok(Some(user))
            }
            None => Ok(None),
        }
    }

    /// 通过邮箱检索用户
    async fn find_by_email(&mut self, email: &Email) -> Result<Option<User>, UserRepoError> {
        let row_opt = sqlx::query_as!(
            UserRow,
            "SELECT * FROM sys_user WHERE email = $1 AND deleted_at IS NULL",
            email.as_str()
        )
        .fetch_optional(&mut **self.tx)
        .await
        .map_err(Self::map_sqlx_error)?;

        match row_opt {
            Some(row) => {
                let user = self.assemble_user(row).await?;
                Ok(Some(user))
            }
            None => Ok(None),
        }
    }

    /// username 唯一性检查
    async fn exists_by_username(&mut self, username: &str) -> Result<bool, UserRepoError> {
        let exists = sqlx::query_scalar!(
                r#"SELECT EXISTS(SELECT 1 FROM sys_user WHERE username = $1 AND deleted_at IS NULL) as "exists!""#,
                username
            )
            .fetch_one(&mut **self.tx)
            .await
            .map_err(Self::map_sqlx_error)?;

        Ok(exists)
    }

    /// email 唯一性检查
    async fn exists_by_email(&mut self, email: &Email) -> Result<bool, UserRepoError> {
        let exists = sqlx::query_scalar!(
                r#"SELECT EXISTS(SELECT 1 FROM sys_user WHERE email = $1 AND deleted_at IS NULL) as "exists!""#,
                email.as_str()
            )
            .fetch_one(&mut **self.tx)
            .await
            .map_err(Self::map_sqlx_error)?;

        Ok(exists)
    }

    /// mobile 唯一性检查
    async fn exists_by_mobile(&mut self, mobile: &Mobile) -> Result<bool, UserRepoError> {
        let exists = sqlx::query_scalar!(
                r#"SELECT EXISTS(SELECT 1 FROM sys_user WHERE mobile = $1 AND deleted_at IS NULL) as "exists!""#,
                mobile.as_str()
            )
            .fetch_one(&mut **self.tx)
            .await
            .map_err(Self::map_sqlx_error)?;

        Ok(exists)
    }

    /// 自增工号累计生成
    async fn get_next_staff_no(&mut self) -> Result<StaffNo, UserRepoError> {
        // 全局按工号倒序，施加悲观锁 FOR UPDATE。
        let max_staff_no_opt: Option<String> = sqlx::query_scalar!(
            r#"
            SELECT staff_no FROM sys_user
            WHERE deleted_at IS NULL
            ORDER BY staff_no DESC
            LIMIT 1
            FOR UPDATE
            "#
        )
        .fetch_optional(&mut **self.tx)
        .await
        .map_err(Self::map_sqlx_error)?;

        // 安全计算下一个全局计数
        let next_seq = match max_staff_no_opt {
            Some(max_no) => {
                if let Some(last_dash_idx) = max_no.rfind('-') {
                    max_no[last_dash_idx + 1..].parse::<i32>().unwrap_or(0) + 1
                } else {
                    1
                }
            }
            None => 1, // 库中开天辟地第一条记录
        };

        // 完美组装返回：STAFF-000001 强类型值对象
        Ok(StaffNo::reconstitute(format!("STAFF-{:06}", next_seq)))
    }
}
