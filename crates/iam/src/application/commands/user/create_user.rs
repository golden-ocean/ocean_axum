use shared::prelude::Uuid;

use crate::application::error::UserAppError;
use crate::domain::entity::User;
use crate::domain::error::UserDomainError;
use crate::domain::repository::UnitOfWorkManager;
use crate::domain::value_object::common::OrganizationId;
use crate::domain::value_object::user::{Email, Mobile};

pub struct CreateUserCommand {
    pub username: String,
    pub password_hash: String,
    pub salt: String,
    pub name: String,
    pub email: String,
    pub mobile: String,
    pub organization_id: Option<Uuid>,
    pub operator_id: Option<Uuid>,
}

pub async fn handle_create_user(
    uow_manager: &(dyn UnitOfWorkManager + Send + Sync),
    cmd: CreateUserCommand,
) -> Result<Uuid, UserAppError> {
    let email_vo = Email::new(cmd.email)?;
    let mobile_vo = Mobile::new(cmd.mobile)?;
    let org_id_vo = cmd.organization_id.map(OrganizationId::from);

    let mut uow = uow_manager.start_work().await?;

    // 检查用户名是否唯一
    if uow.user_repo().exists_by_username(&cmd.username).await? {
        return Err(UserDomainError::UsernameAlreadyExists.into());
    }
    // 检查邮箱是否唯一
    if uow.user_repo().exists_by_email(&email_vo).await? {
        return Err(UserDomainError::EmailAlreadyExists.into());
    }
    // 检查手机号是否唯一
    if uow.user_repo().exists_by_mobile(&mobile_vo).await? {
        return Err(UserDomainError::MobileAlreadyExists.into());
    }
    // 5. 核心并发防撞盾：从同一个大事务内部，执行 FOR UPDATE 查出最新全局序号，拼出强类型 StaffNo VO
    let staff_no_vo = uow.user_repo().get_next_staff_no().await?;

    let new_user = User::new(
        cmd.username,
        cmd.password_hash,
        cmd.salt,
        staff_no_vo,
        cmd.name,
        email_vo,
        mobile_vo,
        org_id_vo,
        cmd.operator_id,
    );

    let new_user_id = new_user.id().value();

    uow.user_repo().save(&new_user).await?;

    uow.commit().await?;

    tracing::info!(
        "【IAM写用例大获成功】新账号全量原子落库成功! 产生的UserId: {:?}, 绑定的全局自增工号: {:?}, 操作人: {:?}",
        new_user_id,
        new_user.staff_no().value(),
        cmd.operator_id
    );

    Ok(new_user_id)
}
