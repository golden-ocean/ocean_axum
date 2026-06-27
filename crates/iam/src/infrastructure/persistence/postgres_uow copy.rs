// use async_trait::async_trait;
// use sqlx::{Postgres, Transaction};

// use crate::domain::repository::UnitOfWork;
// use crate::domain::repository::error::UserRepoError;
// use crate::infrastructure::persistence::PostgresUserRepository;

// pub struct PostgresUnitOfWork {
//     tx: Transaction<'static, Postgres>, // 持有独立长周期事务
// }

// impl PostgresUnitOfWork {
//     pub fn new(tx: Transaction<'static, Postgres>) -> Self {
//         Self { tx }
//     }
// }

// #[async_trait]
// impl UnitOfWork for PostgresUnitOfWork {
//     type UserRepo = PostgresUserRepository<'static>;

//     fn users(&mut self) -> &mut Self::UserRepo {
//         &mut PostgresUserRepository::new(&mut self.tx)
//     }

//     async fn commit(mut self) -> Result<(), UserRepoError> {
//         // 消耗掉自己，统一提交事务，物理释放 FOR UPDATE 行级锁
//         self.tx
//             .commit()
//             .await
//             .map_err(|_| UserRepoError::DatabaseError);

//         Ok(())
//     }
// }
