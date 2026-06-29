#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkStatus {
    InService, // 在职
    OnLeave,   // 休假
    Resigned,  // 离职
}

impl Default for WorkStatus {
    fn default() -> Self {
        Self::InService
    }
}

impl WorkStatus {
    pub fn is_resigned(&self) -> bool {
        matches!(self, WorkStatus::Resigned)
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            WorkStatus::InService => "InService",
            WorkStatus::OnLeave => "OnLeave",
            WorkStatus::Resigned => "Resigned",
        }
    }

    pub fn from_storage(raw: &str) -> Self {
        match raw {
            "InService" => WorkStatus::InService,
            "OnLeave" => WorkStatus::OnLeave,
            _ => WorkStatus::Resigned,
        }
    }
}
