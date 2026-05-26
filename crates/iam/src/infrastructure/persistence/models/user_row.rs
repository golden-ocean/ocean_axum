use std::convert::TryFrom;

use shared::prelude::{AuditMetadata, DateTime, SoftDelete, Status, Utc, Uuid};

use crate::domain::entity::User;
use crate::domain::error::UserDomainError;
use crate::domain::value_object::common::{OrganizationId, PositionId, UserId};
use crate::domain::value_object::user::{DataScope, Email, Gender, Mobile, WorkStatus};

#[derive(Debug, sqlx::FromRow)]
pub struct UserRow {
    pub id: Uuid,
    pub username: String,
    pub employee_code: String,
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
            id: user.id.value(),
            username: user.username.clone(),
            employee_code: user.employee_code.clone(),
            name: user.name.clone(),
            email: user.email.as_str().to_string(),
            mobile: user.mobile.as_str().to_string(),
            gender: user.gender.as_str().to_string(),
            birthday: user.birthday,
            avatar: user.avatar.clone(),

            password_hash: user.password_hash().to_string(),
            salt: user.salt().to_string(),
            password_updated_at: user.password_updated_at,

            work_status: user.work_status.as_str().to_string(),
            data_scope: user.data_scope.as_string(),
            is_builtin: user.is_builtin,
            sort: user.sort,
            remark: user.remark.clone(),
            status: user.status().to_str().to_string(),

            organization_id: user.organization_id.map(|id| id.value()),
            position_id: user.position_id.map(|id| id.value()),

            created_at: user.audit.created_at,
            updated_at: user.audit.updated_at,
            created_by: user.audit.created_by,
            updated_by: user.audit.updated_by,
            deleted_at: user.soft_delete.deleted_at,
            deleted_by: user.soft_delete.deleted_by,
        }
    }
}

impl TryFrom<UserRow> for User {
    type Error = UserDomainError;

    fn try_from(po: UserRow) -> Result<Self, Self::Error> {
        let email_vo =
            Email::new(po.email).map_err(|e| UserDomainError::InvalidFields(e.to_string()))?;
        let mobile_vo =
            Mobile::new(po.mobile).map_err(|e| UserDomainError::InvalidFields(e.to_string()))?;
        let status_vo =
            Status::try_from(po.status.as_str()).map_err(|e| UserDomainError::InvalidFields(e))?;

        let gender_vo = Gender::from_str(&po.gender);
        let work_status_vo = WorkStatus::from_str(&po.work_status);
        let data_scope_vo = DataScope::from_str(&po.data_scope);

        let org_id_vo = po.organization_id.map(OrganizationId::from);
        let pos_id_vo = po.position_id.map(PositionId::from);

        let audit = AuditMetadata {
            created_at: po.created_at,
            updated_at: po.updated_at,
            created_by: po.created_by,
            updated_by: po.updated_by,
        };
        let soft_delete = SoftDelete {
            deleted_at: po.deleted_at,
            deleted_by: po.deleted_by,
        };

        Ok(User::reconstitute(
            UserId::from(po.id),
            po.username,
            po.password_hash,
            po.salt,
            po.password_updated_at,
            po.emp_no,
            po.name,
            email_vo,
            mobile_vo,
            gender_vo,
            po.birthday,
            po.avatar,
            work_status_vo,
            data_scope_vo,
            po.is_builtin,
            po.sort,
            po.remark,
            status_vo,
            org_id_vo,
            pos_id_vo,
            vec![], // role_ids 在此需要通过联表查询获得，如果这里是简单还原，暂时为空或在此处添加关联查询逻辑
            audit,
            soft_delete,
        ))
    }
}
