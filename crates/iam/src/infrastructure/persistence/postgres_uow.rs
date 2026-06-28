use async_trait::async_trait;
use sqlx::{PgPool, Postgres, Transaction};

use crate::domain::repository::error::UserRepoError;
use crate::domain::repository::{UnitOfWork, UnitOfWorkManager, UserRepository};
use crate::infrastructure::persistence::PostgresUserRepository;

/// 物理工作单元管理器适配器 (Adapter)
pub struct PostgresUnitOfWorkManager {
    pool: PgPool,
}

impl PostgresUnitOfWorkManager {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UnitOfWorkManager for PostgresUnitOfWorkManager {
    async fn start_work(&self) -> Result<Box<dyn UnitOfWork>, UserRepoError> {
        let tx = self
            .pool
            .begin()
            .await
            .map_err(|e| UserRepoError::Unexpected(e.to_string()))?;

        let uow = PostgresUnitOfWork::new(tx);
        Ok(Box::new(uow))
    }
}

/// 物理活跃工作单元会话适配器 (Adapter)
pub struct PostgresUnitOfWork {
    tx: Transaction<'static, Postgres>,
}

impl PostgresUnitOfWork {
    pub fn new(tx: Transaction<'static, Postgres>) -> Self {
        Self { tx }
    }
}

#[async_trait]
impl UnitOfWork for PostgresUnitOfWork {
    fn user_repo<'a>(&'a mut self) -> Box<dyn UserRepository + 'a> {
        Box::new(PostgresUserRepository::new(&mut self.tx))
    }

    async fn commit(self: Box<Self>) -> Result<(), UserRepoError> {
        self.tx
            .commit()
            .await
            .map_err(|e| UserRepoError::Unexpected(e.to_string()))?;
        Ok(())
    }
}
