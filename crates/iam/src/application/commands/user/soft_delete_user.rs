use crate::application::error::IamAppError;
use crate::application::ports::outbound::persistence::UnitOfWorkManager;
use crate::domain::error::UserDomainError;
use crate::domain::value_object::common::UserId;

pub struct SoftDeleteUserCommand {
    pub id: shared::prelude::Uuid,
    pub operator_id: shared::prelude::Uuid,
}

pub async fn handle_soft_delete_user(
    uow_manager: &(dyn UnitOfWorkManager + Send + Sync),
    cmd: SoftDeleteUserCommand,
) -> Result<(), IamAppError> {
    let mut uow = uow_manager.start_work().await?;
    let mut user = uow
        .user_repo()
        .find_by_id(&UserId::from(cmd.id))
        .await?
        .ok_or(UserDomainError::UserNotFound)?;

    user.soft_delete(cmd.operator_id)?;

    uow.user_repo().save(&user).await?;
    uow.commit().await?;

    Ok(())
}
