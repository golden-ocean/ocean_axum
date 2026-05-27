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
    repo: &impl UserRepository,
    cmd: CreateUserCommand,
) -> Result<Uuid, UserAppError> {
    if repo.exists_by_username(&cmd.username).await? {
        return Err(UserDomainError::UsernameAlreadyExists.into());
    }
    if repo.exists_by_email(&cmd.email).await? {
        return Err(UserDomainError::EmailAlreadyExists.into());
    }
    if repo.exists_by_mobile(&cmd.mobile).await? {
        return Err(UserDomainError::MobileAlreadyExists.into());
    }

    let email_vo = Email::new(cmd.email)?;
    let mobile_vo = Mobile::new(cmd.mobile)?;
    let org_id_vo = cmd.organization_id.map(OrganizationId::from);

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

    Ok(new_user_id)
}
