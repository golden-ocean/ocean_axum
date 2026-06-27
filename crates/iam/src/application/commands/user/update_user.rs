use crate::application::error::UserAppError;
use crate::domain::error::UserDomainError;
use crate::domain::repository::UserRepository;
use crate::domain::value_object::common::UserId;

pub struct UpdateUserCommand {
    pub id: shared::prelude::Uuid,
    pub username: String,
    pub name: String,
    pub email: String,
    pub mobile: String,
    pub operator_id: shared::prelude::Uuid,
}

pub async fn handle_update_user(
    repo: &(dyn UserRepository + Send + Sync),
    cmd: UpdateUserCommand,
) -> Result<(), UserAppError> {
    let target_email = crate::domain::value_object::user::Email::new(cmd.email)?;
    let target_mobile = crate::domain::value_object::user::Mobile::new(cmd.mobile)?;

    // 获取数据库的user
    let mut user = repo
        .find_by_id(&UserId::from(cmd.id))
        .await?
        .ok_or(UserDomainError::UserNotFound)?;

    // 检查唯一性
    if user.username() != cmd.username {
        if repo.exists_by_username(&cmd.username).await? {
            return Err(UserDomainError::UsernameAlreadyExists.into());
        }
    }
    if user.email().as_str() != target_email.as_str() {
        if repo.exists_by_email(target_email.as_str()).await? {
            return Err(UserDomainError::EmailAlreadyExists.into());
        }
    }

    if user.mobile().as_str() != target_mobile.as_str() {
        if repo.exists_by_mobile(target_mobile.as_str()).await? {
            return Err(UserDomainError::MobileAlreadyExists.into());
        }
    }

    user.update_profile(cmd.name, target_email, target_mobile, cmd.operator_id)?;

    repo.save(&user).await?;

    Ok(())
}
