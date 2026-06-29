use shared::prelude::{AuditMetadata, DeleteMetadata};

use crate::application::ports::outbound::persistence::UserRepositoryError;
use crate::domain::entity::user::User;
use crate::domain::value_object::common::{OrganizationId, PositionId, RoleId, UserId};

use crate::infrastructure::outbound::persistence::model::UserRow;

pub struct UserMapper;

impl UserMapper {
    pub fn to_entity(row: UserRow, role_ids: Vec<RoleId>) -> Result<User, UserRepositoryError> {
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

        let user = User::from_storage(
            UserId::from(row.id),
            row.username,
            row.password_hash,
            row.password_updated_at,
            row.staff_no,
            row.name,
            row.email,
            row.mobile,
            row.gender,
            row.birthday,
            row.avatar,
            row.work_status,
            row.data_scope,
            row.is_builtin,
            row.sort,
            row.remark,
            row.status,
            row.organization_id.map(OrganizationId::from),
            row.position_id.map(PositionId::from),
            role_ids,
            audit_metadata,
            delete_metadata,
        )?;

        Ok(user)
    }

    pub fn to_row(entity: &User) -> UserRow {
        UserRow {
            id: entity.id().value(),
            username: entity.username().to_owned(),
            staff_no: entity.staff_no().value().to_owned(),
            name: entity.name().to_owned(),
            email: entity.email().as_str().to_owned(),
            mobile: entity.mobile().as_str().to_owned(),
            gender: entity.gender().as_str().to_owned(),
            birthday: entity.birthday(),
            avatar: entity.avatar().map(str::to_owned),

            password_hash: entity.password_hash().as_str().to_owned(),
            password_updated_at: entity.password_updated_at(),

            work_status: entity.work_status().as_str().to_owned(),
            data_scope: entity.data_scope().as_string(),
            is_builtin: entity.is_builtin(),
            sort: entity.sort(),
            remark: entity.remark().map(str::to_owned),

            status: entity.status().as_str().to_owned(),

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
