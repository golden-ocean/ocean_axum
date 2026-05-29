use shared::prelude::Uuid;

use crate::application::error::UserAppError;
use crate::domain::error::UserDomainError;
use crate::domain::repository::UserRepository;
use crate::domain::value_object::common::UserId;

pub struct UpdateUserCommand {
    pub id: Uuid,
    pub username: String,
    pub name: String,
    pub email: String,
    pub mobile: String,
    pub operator_id: Uuid,
}

pub async fn handle_update_user(
    repo: &impl UserRepository,
    cmd: UpdateUserCommand,
) -> Result<(), UserAppError> {
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
    if user.email().as_str() != cmd.email {
        if repo.exists_by_email(&cmd.email).await? {
            return Err(UserDomainError::EmailAlreadyExists.into());
        }
    }
    if user.mobile().as_str() != cmd.mobile {
        if repo.exists_by_mobile(&cmd.mobile).await? {
            return Err(UserDomainError::MobileAlreadyExists.into());
        }
    }

    // 3. 执行领域动作 (充血模型修改)
    // user.update_profile(cmd.name, cmd.email, cmd.mobile, cmd.operator_id)?;

    // 4. 持久化 (此时 save 调用的是主键 id 的 Upsert)
    repo.save(&user).await?;
    Ok(())
}
