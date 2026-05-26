#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Status {
    /// 启用
    #[default]
    Enabled,

    /// 停用
    Disabled,
}

impl Status {
    /// 判断是否为启用状态
    pub fn is_enabled(&self) -> bool {
        matches!(self, Self::Enabled)
    }

    /// 判断是否为停用状态
    pub fn is_disabled(&self) -> bool {
        matches!(self, Self::Disabled)
    }

    /// 显式转换为静态字符串（常用于日志打印或原生 SQL 组装）
    pub fn to_str(&self) -> &'static str {
        match self {
            Self::Enabled => "Enabled",
            Self::Disabled => "Disabled",
        }
    }
}

/// 数据库或外部传输层转领域状态
impl TryFrom<&str> for Status {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.trim().to_lowercase().as_str() {
            "Enabled" => Ok(Self::Enabled),
            "Disabled" => Ok(Self::Disabled),
            _ => Err(format!("非法的数据库状态值: '{}'", value)),
        }
    }
}

/// 支持从 String 直接转换（方便 sqlx 从数据库文本字段映射）
impl TryFrom<String> for Status {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}
