use async_trait::async_trait;

use crate::domain::entity::User;
use crate::domain::repository::error::UserRepoError;
use crate::domain::value_object::common::UserId;

#[async_trait]
pub trait UserRepository: Send + Sync {
    /// Upsert
    async fn save(&self, user: &User) -> Result<(), UserRepoError>;

    /// 通过 ID 删除 物理删除
    async fn delete(&self, user_id: &UserId) -> Result<(), UserRepoError>;

    /// 通过 ID 查找用户
    async fn find_by_id(&self, id: &UserId) -> Result<Option<User>, UserRepoError>;

    /// 通过用户名查找用户 (登录时常用)
    async fn find_by_username(&self, username: &str) -> Result<Option<User>, UserRepoError>;

    /// 通过手机号查找用户
    async fn find_by_mobile(&self, mobile: &str) -> Result<Option<User>, UserRepoError>;

    /// 通过邮箱查找用户
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, UserRepoError>;

    /// 检查用户名是否已存在 (注册防重用)
    async fn exists_by_username(&self, username: &str) -> Result<bool, UserRepoError>;
}
