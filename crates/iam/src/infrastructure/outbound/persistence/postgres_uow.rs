use async_trait::async_trait;
use sqlx::{PgPool, Postgres, Transaction};

use crate::application::ports::outbound::persistence::{
    UnitOfWork, UnitOfWorkManager, UserRepository, UserRepositoryError,
};
use crate::infrastructure::outbound::persistence::PostgresUserRepository;

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
    async fn start_work(&self) -> Result<Box<dyn UnitOfWork>, UserRepositoryError> {
        let tx = self
            .pool
            .begin()
            .await
            .map_err(|e| UserRepositoryError::Unexpected(e.to_string()))?;

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

    async fn commit(self: Box<Self>) -> Result<(), UserRepositoryError> {
        self.tx
            .commit()
            .await
            .map_err(|e| UserRepositoryError::Unexpected(e.to_string()))?;
        Ok(())
    }

    async fn rollback(self: Box<Self>) -> Result<(), UserRepositoryError> {
        self.tx
            .rollback()
            .await
            .map_err(|e| UserRepositoryError::Unexpected(e.to_string()))?;
        Ok(())
    }
}
