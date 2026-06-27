use async_trait::async_trait;

use crate::domain::entity::User;
use crate::domain::repository::error::UserRepoError;
use crate::domain::value_object::common::UserId;
use crate::domain::value_object::user::{Email, Mobile, StaffNo};

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn save(&mut self, user: &User) -> Result<(), UserRepoError>;
    async fn remove(&mut self, user_id: &UserId) -> Result<(), UserRepoError>;

    async fn find_by_id(&mut self, user_id: &UserId) -> Result<Option<User>, UserRepoError>;
    async fn find_by_username(&mut self, username: &str) -> Result<Option<User>, UserRepoError>;
    async fn find_by_email(&mut self, email: &Email) -> Result<Option<User>, UserRepoError>;
    async fn find_by_mobile(&mut self, mobile: &Mobile) -> Result<Option<User>, UserRepoError>;

    async fn exists_by_username(&mut self, username: &str) -> Result<bool, UserRepoError>;
    async fn exists_by_email(&mut self, email: &Email) -> Result<bool, UserRepoError>;
    async fn exists_by_mobile(&mut self, mobile: &Mobile) -> Result<bool, UserRepoError>;

    async fn get_next_staff_no(&mut self) -> Result<StaffNo, UserRepoError>;
}
