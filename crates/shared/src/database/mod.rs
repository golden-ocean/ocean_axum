#[derive(Debug, Clone, serde::Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum DbDriver {
    Postgres,
    Sqlite,
}

/// 🌟 运行时数据库连接池盲盒
#[derive(Debug, Clone)]
pub enum AppDbPool {
    Postgres(sqlx::PgPool),
    Sqlite(sqlx::SqlitePool),
}

/// 🌟 运行时活跃事务盲盒
pub enum AppTransaction {
    Postgres(sqlx::Transaction<'static, sqlx::Postgres>),
    Sqlite(sqlx::Transaction<'static, sqlx::Sqlite>),
}
