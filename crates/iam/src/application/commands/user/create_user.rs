use shared::prelude::Uuid;

use crate::application::error::UserAppError;
use crate::domain::entity::User;
use crate::domain::error::UserDomainError;
use crate::domain::repository::UserRepository;
use crate::domain::value_object::common::OrganizationId;
use crate::domain::value_object::user::{Email, Mobile};

pub struct CreateUserCommand {
    pub username: String,
    pub password_hash: String,
    pub salt: String,
    pub emp_no: String,
    pub name: String,
    pub email: String,
    pub mobile: String,
    pub organization_id: Option<Uuid>,
    pub operator_id: Option<Uuid>,
}

pub async fn handle_create_user(
    repo: &(dyn UserRepository + Send + Sync),
    cmd: CreateUserCommand,
) -> Result<Uuid, UserAppError> {
    let email_vo = Email::new(cmd.email)?;
    let mobile_vo = Mobile::new(cmd.mobile)?;
    let org_id_vo = cmd.organization_id.map(OrganizationId::from);

    // 检查用户名是否唯一
    if repo.exists_by_username(&cmd.username).await? {
        return Err(UserDomainError::UsernameAlreadyExists.into());
    }
    // 检查邮箱是否唯一
    if repo.exists_by_email(email_vo.as_str()).await? {
        return Err(UserDomainError::EmailAlreadyExists.into());
    }
    // 检查手机号是否唯一
    if repo.exists_by_mobile(mobile_vo.as_str()).await? {
        return Err(UserDomainError::MobileAlreadyExists.into());
    }

    let new_user = User::new(
        cmd.username,
        cmd.password_hash,
        cmd.salt,
        cmd.emp_no,
        cmd.name,
        email_vo,
        mobile_vo,
        org_id_vo,
        cmd.operator_id,
    );

    let new_user_id = new_user.id().value();

    repo.save(&new_user).await?;

    tracing::info!(
        "【IAM用例成功】用户创建成功! 产生的UserId: {:?}, 操作人: {:?}",
        new_user_id,
        cmd.operator_id
    );

    Ok(new_user_id)
}
