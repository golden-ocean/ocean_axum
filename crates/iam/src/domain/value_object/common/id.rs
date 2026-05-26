use shared::prelude::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UserId(Uuid);

impl UserId {
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }
    pub fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
    pub fn value(&self) -> Uuid {
        self.0
    }
}

impl std::fmt::Display for UserId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RoleId(Uuid);

impl RoleId {
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }
    pub fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
    pub fn value(&self) -> Uuid {
        self.0
    }
}

impl std::fmt::Display for RoleId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PermissionId(Uuid);

impl PermissionId {
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }
    pub fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
    pub fn value(&self) -> Uuid {
        self.0
    }
}

impl std::fmt::Display for PermissionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// 外部模块 (ORG等) 的防腐 ID 引用
// 模块解耦
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OrganizationId(Uuid);

impl OrganizationId {
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }
    pub fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
    pub fn value(&self) -> Uuid {
        self.0
    }
}

impl std::fmt::Display for OrganizationId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PositionId(Uuid);

impl PositionId {
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }
    pub fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
    pub fn value(&self) -> Uuid {
        self.0
    }
}

impl std::fmt::Display for PositionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
