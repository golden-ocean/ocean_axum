#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Gender {
    Unknown,
    Male,
    Female,
}

impl Default for Gender {
    fn default() -> Self {
        Self::Unknown
    }
}

impl Gender {
    /// 转成数据库存储的字符串
    pub fn as_str(&self) -> &'static str {
        match self {
            Gender::Unknown => "Unknown",
            Gender::Male => "Male",
            Gender::Female => "Female",
        }
    }

    /// 从数据库字符串解析
    pub fn from_str(s: &str) -> Self {
        match s {
            "Male" => Gender::Male,
            "Female" => Gender::Female,
            _ => Gender::Unknown,
        }
    }
}
