use crate::domain::repository::UserRepository;
use crate::domain::repository::error::UserRepoError;
use async_trait::async_trait;

#[async_trait]
pub trait UnitOfWork: Send {
    /// 具象仓储成员
    type UserRepo: UserRepository;
    // type RoleRepo: RoleRepository;

    /// 获取受当前事务生命周期保护的强类型 Repository 引用
    fn users(&mut self) -> &mut Self::UserRepo;

    async fn commit(self: Box<Self>) -> Result<(), UserRepoError>;
}

#[async_trait]
pub trait UnitOfWorkManager: Send + Sync {
    type TxWork: UnitOfWork;

    /// 开启数据库事务
    async fn start_work(&self) -> Result<Box<dyn UnitOfWork>, UserRepoError>;
}
