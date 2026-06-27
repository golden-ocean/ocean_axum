use shared::prelude::{DateTime, NaiveDate, Utc, Uuid};

#[derive(Debug, sqlx::FromRow)]
pub struct UserRow {
    pub id: Uuid,
    pub username: String,
    pub staff_no: String,
    pub name: String,
    pub email: String,
    pub mobile: String,
    pub gender: String,
    pub birthday: Option<NaiveDate>,
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
