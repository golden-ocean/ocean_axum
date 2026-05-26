use std::convert::TryFrom;

use shared::prelude::{AuditMetadata, DateTime, DeleteMetadata, Status, Utc, Uuid};

use crate::domain::entity::User;
use crate::domain::error::UserDomainError;
use crate::domain::value_object::common::{OrganizationId, PositionId, UserId};
use crate::domain::value_object::user::{DataScope, Email, Gender, Mobile, WorkStatus};

#[derive(Debug, sqlx::FromRow)]
pub struct UserRow {
    pub id: Uuid,
    pub username: String,
    pub emp_no: String,
    pub name: String,
    pub email: String,
    pub mobile: String,
    pub gender: String,
    pub birthday: Option<DateTime<Utc>>,
    pub avatar: Option<String>,

    pub password_hash: String,
    pub salt: String,
    pub password_updated_at: DateTime<Utc>,

    pub work_status: String,
    pub data_scope: String,
    pub is_builtin: bool,
    pub sort: i32,
    pub remark: Option<String>,
    pub status: String,

    pub organization_id: Option<Uuid>,
    pub position_id: Option<Uuid>,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub deleted_by: Option<Uuid>,
}

impl From<&User> for UserRow {
    fn from(user: &User) -> Self {
        Self {
            id: user.id().value(),
            username: user.username().to_string(),
            emp_no: user.emp_no().to_string(),
            name: user.name().to_string(),
            email: user.email().as_str().to_string(),
            mobile: user.mobile().as_str().to_string(),
            gender: user.gender().as_str().to_string(),
            birthday: user.birthday(),
            avatar: user.avatar().map(String::from),

            password_hash: user.password_hash().to_string(),
            salt: user.salt().to_string(),
            password_updated_at: user.password_updated_at(),

            work_status: user.work_status().as_str().to_string(),
            data_scope: user.data_scope().as_string(),
            is_builtin: user.is_builtin(),
            sort: user.sort(),
            remark: user.remark().map(String::from),
            status: user.status().to_str().to_string(),

            organization_id: user.organization_id().map(|id| id.value()),
            position_id: user.position_id().map(|id| id.value()),

            created_at: user.audit_metadata().created_at,
            updated_at: user.audit_metadata().updated_at,
            created_by: user.audit_metadata().created_by,
            updated_by: user.audit_metadata().updated_by,
            deleted_at: user.delete_metadata().deleted_at,
            deleted_by: user.delete_metadata().deleted_by,
        }
    }
}

impl TryFrom<UserRow> for User {
    type Error = UserDomainError;

    fn try_from(row: UserRow) -> Result<Self, Self::Error> {
        let email_vo =
            Email::new(row.email).map_err(|e| UserDomainError::InvalidFields(e.to_string()))?;
        let mobile_vo =
            Mobile::new(row.mobile).map_err(|e| UserDomainError::InvalidFields(e.to_string()))?;
        let status_vo =
            Status::try_from(row.status.as_str()).map_err(|e| UserDomainError::InvalidFields(e))?;

        let gender_vo = Gender::from_str(&row.gender);
        let work_status_vo = WorkStatus::from_str(&row.work_status);
        let data_scope_vo = DataScope::from_str(&row.data_scope);

        let org_id_vo = row.organization_id.map(OrganizationId::from);
        let pos_id_vo = row.position_id.map(PositionId::from);

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
            row.emp_no,
            row.name,
            email_vo,
            mobile_vo,
            gender_vo,
            row.birthday,
            row.avatar,
            work_status_vo,
            data_scope_vo,
            row.is_builtin,
            row.sort,
            row.remark,
            status_vo,
            org_id_vo,
            pos_id_vo,
            vec![], // 角色需要单独查询
            audit_metadata,
            delete_metadata,
        ))
    }
}
