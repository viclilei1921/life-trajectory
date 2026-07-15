use std::path::Path;

use log::error;
use sqlx::SqlitePool;

use super::error::AppError;

/// 初始化 SQLite 连接池：建目录 → 连接数据库 → 执行迁移。
///
/// 在应用启动时调用一次，返回的 pool 注入 [`AppState`](crate::server::state::AppState) 供全局复用。
pub async fn init_pool(db_path: &Path) -> Result<SqlitePool, AppError> {
  // 确保数据库文件的父目录存在（首次启动时 app_data_dir 可能尚未创建）
  if let Some(parent) = db_path.parent() {
    std::fs::create_dir_all(parent).map_err(|e| {
      error!("failed to create db directory at {}: {e}", parent.display());
      AppError::internal(e.to_string())
    })?;
  }

  // mode=rwc：不存在则创建文件，存在则读写打开
  let url = format!("sqlite:{}?mode=rwc", db_path.display());
  let pool = SqlitePool::connect(&url).await.map_err(|e| {
    error!("failed to connect sqlite at {}: {e}", db_path.display());
    AppError::internal(e.to_string())
  })?;

  // 编译期嵌入 migrations/ 下的 SQL，按版本号顺序执行未应用的迁移
  sqlx::migrate!("./migrations")
    .run(&pool)
    .await
    .map_err(|e| {
      error!("failed to run db migrations: {e}");
      AppError::internal(e.to_string())
    })?;

  Ok(pool)
}
