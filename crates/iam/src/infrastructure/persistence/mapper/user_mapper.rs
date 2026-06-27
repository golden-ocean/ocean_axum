use shared::prelude::{AuditMetadata, DeleteMetadata, Status};

use crate::domain::entity::user::User;
use crate::domain::repository::error::UserRepoError;
use crate::domain::value_object::common::{OrganizationId, PositionId, RoleId, UserId};
use crate::domain::value_object::user::{DataScope, Email, Gender, Mobile, StaffNo, WorkStatus};
use crate::infrastructure::persistence::model::UserRow;

pub struct UserMapper;

impl UserMapper {
    pub fn to_entity(row: UserRow, role_ids: Vec<RoleId>) -> Result<User, UserRepoError> {
        let staff_no = StaffNo::new(row.staff_no).map_err(|e| {
            UserRepoError::DataInconsistent(format!("数据库中的员工工号格式已损毁: {:?}", e))
        })?;
        let email = Email::new(row.email).map_err(|e| {
            UserRepoError::DataInconsistent(format!("数据库中的邮箱格式已损毁: {:?}", e))
        })?;

        let mobile = Mobile::new(row.mobile).map_err(|e| {
            UserRepoError::DataInconsistent(format!("数据库中的手机号格式已损毁: {:?}", e))
        })?;

        let status = Status::from_str(&row.status).ok_or_else(|| {
            UserRepoError::DataInconsistent(format!(
                "数据库中存在非法的用户状态值: '{}'",
                row.status
            ))
        })?;

        let gender = Gender::from_str(&row.gender);
        let work_status = WorkStatus::from_str(&row.work_status);
        let data_scope = DataScope::from_str(&row.data_scope);

        let audit_metadata = AuditMetadata {
            created_at: row.created_at,
            updated_at: row.updated_at,
            created_by: row.created_by,
            updated_by: row.updated_by,
        };

        let delete_metadata = DeleteMetadata {
            deleted_at: row.deleted_at,
            deleted_by: row.deleted_by,
        };

        Ok(User::reconstruct(
            UserId::from(row.id),
            row.username,
            row.password_hash,
            row.salt,
            row.password_updated_at,
            staff_no,
            row.name,
            email,
            mobile,
            gender,
            row.birthday,
            row.avatar,
            work_status,
            data_scope,
            row.is_builtin,
            row.sort,
            row.remark,
            status,
            row.organization_id.map(OrganizationId::from),
            row.position_id.map(PositionId::from),
            role_ids,
            audit_metadata,
            delete_metadata,
        ))
    }

    pub fn to_row(entity: &User) -> UserRow {
        UserRow {
            id: entity.id().value(),
            username: entity.username().to_string(),
            staff_no: entity.staff_no().to_string(),
            name: entity.name().to_string(),
            email: entity.email().as_str().to_string(),
            mobile: entity.mobile().as_str().to_string(),
            gender: entity.gender().as_str().to_string(),
            birthday: entity.birthday(),
            avatar: entity.avatar().map(|s| s.to_string()),

            password_hash: entity.password_hash().to_string(),
            salt: entity.salt().to_string(),
            password_updated_at: entity.password_updated_at(),

            work_status: entity.work_status().as_str().to_string(),
            data_scope: entity.data_scope().as_string(),
            is_builtin: entity.is_builtin(),
            sort: entity.sort(),
            remark: entity.remark().map(|s| s.to_string()),

            status: entity.status().as_str().to_string(),

            organization_id: entity.organization_id().map(|id| id.value()),
            position_id: entity.position_id().map(|id| id.value()),

            created_at: entity.audit_metadata().created_at,
            updated_at: entity.audit_metadata().updated_at,
            created_by: entity.audit_metadata().created_by,
            updated_by: entity.audit_metadata().updated_by,
            deleted_at: entity.delete_metadata().deleted_at,
            deleted_by: entity.delete_metadata().deleted_by,
        }
    }
}
