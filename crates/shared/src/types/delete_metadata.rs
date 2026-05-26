use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, Default)]
pub struct DeleteMetadata {
    pub deleted_at: Option<DateTime<Utc>>,
    pub deleted_by: Option<Uuid>,
}

impl DeleteMetadata {
    /// 执行软删除动作
    pub fn mark_deleted(&mut self, operator_id: Uuid) {
        self.deleted_at = Some(Utc::now());
        self.deleted_by = Some(operator_id);
    }

    /// 检查是否已删除
    pub fn is_deleted(&self) -> bool {
        self.deleted_at.is_some()
    }
}
