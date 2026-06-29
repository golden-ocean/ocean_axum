use async_trait::async_trait;

use crate::application::ports::outbound::persistence::UserRepositoryError;
use crate::domain::entity::User;
use crate::domain::value_object::common::UserId;
use crate::domain::value_object::user::{Email, Mobile, StaffNo};

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn save(&mut self, user: &User) -> Result<(), UserRepositoryError>;
    async fn remove(&mut self, user_id: &UserId) -> Result<(), UserRepositoryError>;

    async fn find_by_id(&mut self, user_id: &UserId) -> Result<Option<User>, UserRepositoryError>;
    async fn find_by_username(
        &mut self,
        username: &str,
    ) -> Result<Option<User>, UserRepositoryError>;
    async fn find_by_email(&mut self, email: &Email) -> Result<Option<User>, UserRepositoryError>;
    async fn find_by_mobile(
        &mut self,
        mobile: &Mobile,
    ) -> Result<Option<User>, UserRepositoryError>;

    async fn exists_by_username(&mut self, username: &str) -> Result<bool, UserRepositoryError>;
    async fn exists_by_email(&mut self, email: &Email) -> Result<bool, UserRepositoryError>;
    async fn exists_by_mobile(&mut self, mobile: &Mobile) -> Result<bool, UserRepositoryError>;

    async fn get_next_staff_no(&mut self) -> Result<StaffNo, UserRepositoryError>;
}
