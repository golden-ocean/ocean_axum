use shared::prelude::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DataScope {
    All,                     // 全部数据（超级管理员/高管）
    Organization,            // 本部门数据
    OrganizationAndChildren, // 本部门及以下数据权限
    SelfOnly,                // 仅本人数据（默认最小权限）
    Custom(Vec<Uuid>),       // 自定义特定部门的数据权限（比如跨部门项目组）
}

impl Default for DataScope {
    fn default() -> Self {
        Self::SelfOnly
    }
}

impl DataScope {
    /// 一个简单的业务辅助方法，判断当前数据权限是否包含某个特定的部门
    /// (实际的复杂权限过滤通常在 Repository 拼接 SQL 时处理，但内存校验也很有用)
    pub fn contains_org(&self, target_org_id: &Uuid, my_org_id: &Uuid) -> bool {
        match self {
            DataScope::All => true,
            DataScope::Organization => target_org_id == my_org_id,
            DataScope::SelfOnly => false, // 仅限本人时，通常不需要通过部门维度的宽泛校验
            DataScope::Custom(org_ids) => org_ids.contains(target_org_id),
            // OrganizationAndChildren 的精确校验需要查数据库树结构，通常不在纯 VO 里做
            DataScope::OrganizationAndChildren => {
                unimplemented!("Need external tree query service")
            }
        }
    }
}

impl DataScope {
    /// 转成数据库存储的字符串（Custom 使用 JSON）
    pub fn as_string(&self) -> String {
        match self {
            DataScope::All => "All".to_string(),
            DataScope::Organization => "Organization".to_string(),
            DataScope::OrganizationAndChildren => "OrganizationAndChildren".to_string(),
            DataScope::SelfOnly => "SelfOnly".to_string(),
            DataScope::Custom(vec) => serde_json::to_string(vec).unwrap_or("[]".to_string()),
        }
    }

    /// 从数据库字符串解析
    pub fn from_str(s: &str) -> Self {
        match s {
            "All" => DataScope::All,
            "Organization" => DataScope::Organization,
            "OrganizationAndChildren" => DataScope::OrganizationAndChildren,
            "SelfOnly" => DataScope::SelfOnly,
            other => {
                // 尝试解析 JSON Vec<Uuid>
                if let Ok(vec) = serde_json::from_str::<Vec<Uuid>>(other) {
                    DataScope::Custom(vec)
                } else {
                    // 默认最小权限
                    DataScope::SelfOnly
                }
            }
        }
    }
}
