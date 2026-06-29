use shared::error::AppError;
use strum::{EnumDiscriminants, EnumString};
use thiserror::Error;

use crate::domain::value_object::user::{Password, PasswordHash};

#[derive(Debug, Error, EnumDiscriminants)]
#[strum_discriminants(name(PasswordHashErrorCode))]
#[strum_discriminants(derive(EnumString, strum::Display))]
#[strum_discriminants(strum(serialize_all = "SCREAMING_SNAKE_CASE"))]
pub enum PasswordHashError {
    #[error("哈希计算失败 ")]
    HashFailed,
    #[error("验证失败 ")]
    VerifyFailed,
}
impl PasswordHashError {
    pub fn code(&self) -> String {
        format!("IAM_USER_DOMAIN_{}", PasswordHashErrorCode::from(self))
    }
}

impl From<PasswordHashError> for AppError {
    fn from(e: PasswordHashError) -> Self {
        let code = e.code();
        let msg = e.to_string();
        match PasswordHashErrorCode::from(&e) {
            PasswordHashErrorCode::HashFailed => AppError::InternalError(anyhow::anyhow!(e)),
            PasswordHashErrorCode::VerifyFailed => AppError::bad_request(code, msg),
        }
    }
}

#[async_trait::async_trait]
pub trait PasswordHasher: Send + Sync {
    fn hash(&self, password: &Password) -> Result<PasswordHash, PasswordHashError>;
    fn verify(&self, password: &Password, hash: &str) -> Result<(), PasswordHashError>;
}
