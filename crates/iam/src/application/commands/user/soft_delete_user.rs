use shared::prelude::Uuid;

use crate::application::error::UserAppError;
use crate::domain::error::UserDomainError;
use crate::domain::repository::UserRepository;
use crate::domain::value_object::common::UserId;

pub struct SoftDeleteUserCommand {
    pub id: Uuid,
    pub operator_id: Uuid,
}

pub async fn handle_soft_delete_user(
    repo: &impl UserRepository,
    cmd: SoftDeleteUserCommand,
) -> Result<(), UserAppError> {
    let mut user = repo
        .find_by_id(&UserId::from(cmd.id))
        .await?
        .ok_or(UserDomainError::UserNotFound)?;

    user.soft_delete(cmd.operator_id)?;

    repo.save(&user).await?;
    Ok(())
}
