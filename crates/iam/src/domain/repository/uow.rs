use crate::domain::repository::UserRepository;
use crate::domain::repository::error::UserRepoError;
use async_trait::async_trait;

/// 工作单元管理器端口 (UnitOfWorkManager Port)
#[async_trait]
pub trait UnitOfWorkManager: Send + Sync {
    async fn start_work(&self) -> Result<Box<dyn UnitOfWork>, UserRepoError>;
}

/// 活跃工作单元端口 (UnitOfWork Port)
/// 代表一个处于活跃数据库写事务生命周期中的会话
#[async_trait]
pub trait UnitOfWork: Send {
    fn user_repo<'a>(&'a mut self) -> Box<dyn UserRepository + 'a>;

    async fn commit(self: Box<Self>) -> Result<(), UserRepoError>;
}
