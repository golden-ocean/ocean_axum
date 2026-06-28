// 导出领域与接口类型
pub use crate::http::{Page, Pagination, Res};
pub use crate::types::audit_metadata::AuditMetadata;
pub use crate::types::delete_metadata::DeleteMetadata;
pub use crate::types::status::Status;

pub use crate::error::AppError;

// 导出基础设施
// pub use crate::infrastructure::db::init_db;
// pub use crate::infrastructure::log::init_logger;

// 导出常用的第三方库工具 (避免在业务 crate 重复引入)
pub use chrono::{DateTime, NaiveDate, Utc};
pub use serde::{Deserialize, Serialize};
pub use tracing::{debug, error, info, warn};
pub use uuid::Uuid;
