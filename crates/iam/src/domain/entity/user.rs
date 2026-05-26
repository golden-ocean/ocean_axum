use shared::prelude::{AuditMetadata, DateTime, DeleteMetadata, Status, Utc, Uuid};

use crate::domain::error::UserDomainError;
use crate::domain::value_object::common::{OrganizationId, PositionId, RoleId, UserId};
use crate::domain::value_object::user::{DataScope, Email, Gender, Mobile, WorkStatus};

#[derive(Debug, Clone)]
pub struct User {
    id: UserId,
    username: String,
    emp_no: String,
    name: String,
    email: Email,
    mobile: Mobile,
    gender: Gender,
    birthday: Option<DateTime<Utc>>,
    avatar: Option<String>,

    password_hash: String,
    salt: String,
    password_updated_at: DateTime<Utc>,

    work_status: WorkStatus,
    data_scope: DataScope,
    is_builtin: bool,
    sort: i32,
    remark: Option<String>,
    status: Status,

    organization_id: Option<OrganizationId>,
    position_id: Option<PositionId>,
    role_ids: Vec<RoleId>,

    audit_metadata: AuditMetadata,
    delete_metadata: DeleteMetadata,
}

impl User {
    /// 创建新用户
    pub fn new(
        username: String,
        password_hash: String,
        salt: String,
        emp_no: String,
        name: String,
        email: Email,
        mobile: Mobile,
        organization_id: Option<OrganizationId>,
        operator_id: Option<Uuid>,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: UserId::new(),
            username,
            emp_no,
            name,
            email,
            mobile,
            gender: Gender::Unknown, // 默认未知
            birthday: None,
            avatar: None,

            password_hash,
            salt,
            password_updated_at: now,

            work_status: WorkStatus::InService, // 默认在职
            data_scope: DataScope::SelfOnly,    // 默认最小权限
            is_builtin: false,                  // 业务 API 创建的绝对不是内置账号
            sort: 1000,
            remark: None,
            status: Status::Enabled, // 默认启用

            organization_id,
            position_id: None,
            role_ids: vec![], // 默认不分配任何角色

            audit_metadata: AuditMetadata::new(operator_id),
            delete_metadata: DeleteMetadata::default(),
        }
    }

    /// 校验是否允许被修改
    pub fn verify_can_modify(&self) -> Result<(), UserDomainError> {
        if self.is_builtin {
            return Err(UserDomainError::SystemResourceProtected);
        }
        if self.delete_metadata.is_deleted() {
            return Err(UserDomainError::UserNotFound);
        }
        Ok(())
    }

    /// 修改密码
    pub fn change_password(
        &mut self,
        new_hash: String,
        new_salt: String,
        operator_id: Uuid,
    ) -> Result<(), UserDomainError> {
        self.verify_can_modify()?;

        if self.status.is_disabled() {
            return Err(UserDomainError::UserSuspended);
        }

        self.password_hash = new_hash;
        self.salt = new_salt;
        self.password_updated_at = Utc::now();

        self.audit_metadata.update(Some(operator_id));
        Ok(())
    }

    /// 禁用账号
    pub fn disable(&mut self, operator_id: Uuid) -> Result<(), UserDomainError> {
        self.verify_can_modify()?; // 防御：系统内置超管不可被禁用
        self.status = Status::Disabled;
        self.audit_metadata.update(Some(operator_id));
        Ok(())
    }

    /// 启用账号
    pub fn enable(&mut self, operator_id: Uuid) -> Result<(), UserDomainError> {
        self.verify_can_modify()?;
        self.status = Status::Enabled;
        self.audit_metadata.update(Some(operator_id));
        Ok(())
    }

    /// 人事调动 (调整部门和岗位，及数据权限)
    pub fn transfer_position(
        &mut self,
        new_org_id: Option<OrganizationId>,
        new_pos_id: Option<PositionId>,
        new_data_scope: DataScope,
        operator_id: Uuid,
    ) -> Result<(), UserDomainError> {
        self.verify_can_modify()?;

        self.organization_id = new_org_id;
        self.position_id = new_pos_id;
        self.data_scope = new_data_scope;

        self.audit_metadata.update(Some(operator_id));
        Ok(())
    }

    ///  分配/覆盖角色
    pub fn assign_roles(
        &mut self,
        new_role_ids: Vec<RoleId>,
        operator_id: Uuid,
    ) -> Result<(), UserDomainError> {
        self.verify_can_modify()?; // 防御：系统内置账号的角色由底层管控，不允许通过通用接口修改

        self.role_ids = new_role_ids;
        self.audit_metadata.update(Some(operator_id));
        Ok(())
    }

    /// 执行软删除
    pub fn soft_delete(&mut self, operator_id: Uuid) -> Result<(), UserDomainError> {
        self.verify_can_modify()?;

        self.status = Status::Disabled; // 删除同时冻结登录状态
        self.work_status = WorkStatus::Resigned; // 删除视同员工离职

        self.delete_metadata.mark_deleted(operator_id);
        self.audit_metadata.update(Some(operator_id));
        Ok(())
    }

    /// 从数据重建领域实体
    pub fn reconstruct(
        id: UserId,
        username: String,
        password_hash: String,
        salt: String,
        password_updated_at: DateTime<Utc>,
        emp_no: String,
        name: String,
        email: Email,
        mobile: Mobile,
        gender: Gender,
        birthday: Option<DateTime<Utc>>,
        avatar: Option<String>,
        work_status: WorkStatus,
        data_scope: DataScope,
        is_builtin: bool,
        sort: i32,
        remark: Option<String>,
        status: Status,
        organization_id: Option<OrganizationId>,
        position_id: Option<PositionId>,
        role_ids: Vec<RoleId>,
        audit_metadata: AuditMetadata,
        delete_metadata: DeleteMetadata,
    ) -> Self {
        Self {
            id,
            username,
            password_hash,
            salt,
            password_updated_at,
            emp_no,
            name,
            email,
            mobile,
            gender,
            birthday,
            avatar,
            work_status,
            data_scope,
            is_builtin,
            sort,
            remark,
            status,
            organization_id,
            position_id,
            role_ids,
            audit_metadata,
            delete_metadata,
        }
    }
    /// Getters (只读暴露)
    pub fn id(&self) -> &UserId {
        &self.id
    }
    pub fn username(&self) -> &str {
        &self.username
    }
    pub fn emp_no(&self) -> &str {
        &self.emp_no
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn email(&self) -> &Email {
        &self.email
    }
    pub fn mobile(&self) -> &Mobile {
        &self.mobile
    }
    pub fn gender(&self) -> &Gender {
        &self.gender
    }
    pub fn birthday(&self) -> Option<DateTime<Utc>> {
        self.birthday
    }
    pub fn avatar(&self) -> Option<&str> {
        self.avatar.as_deref()
    }
    pub fn password_hash(&self) -> &str {
        &self.password_hash
    }
    pub fn salt(&self) -> &str {
        &self.salt
    }
    pub fn password_updated_at(&self) -> DateTime<Utc> {
        self.password_updated_at
    }
    pub fn work_status(&self) -> &WorkStatus {
        &self.work_status
    }
    pub fn data_scope(&self) -> &DataScope {
        &self.data_scope
    }
    pub fn is_builtin(&self) -> bool {
        self.is_builtin
    }
    pub fn sort(&self) -> i32 {
        self.sort
    }
    pub fn remark(&self) -> Option<&str> {
        self.remark.as_deref()
    }
    pub fn status(&self) -> &Status {
        &self.status
    }
    pub fn organization_id(&self) -> Option<OrganizationId> {
        self.organization_id
    }
    pub fn position_id(&self) -> Option<PositionId> {
        self.position_id
    }
    pub fn role_ids(&self) -> &[RoleId] {
        &self.role_ids
    }
    pub fn audit_metadata(&self) -> &AuditMetadata {
        &self.audit_metadata
    }
    pub fn delete_metadata(&self) -> &DeleteMetadata {
        &self.delete_metadata
    }
}
