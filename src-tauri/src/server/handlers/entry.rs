//! Entry 用例入口：入参校验 + Vault 解锁门禁 + 调用 [`EntryService`]。

use sqlx::SqlitePool;

use crate::server::dto::EntryDto;
use crate::server::error::AppError;
use crate::server::middleware::require_unlocked;
use crate::server::services::{EntryService, VaultService};
use crate::server::services::vault::VaultSession;

/// 创建 Entry
/// 
/// @param pool - 数据库连接池
/// @param session - Vault 会话
/// @param content - Entry 内容
/// @param happened_at - Entry 发生时间
/// @param title - Entry 标题
/// 
/// @return  Entry
pub async fn create(
  pool: &SqlitePool,
  session: &VaultSession,
  content: String,
  happened_at: i64,
  title: Option<String>,
) -> Result<EntryDto, AppError> {
  let config = VaultService::load_config(pool).await?;
  require_unlocked(session, config.mode)?;
  EntryService::create(pool, session, title, content, happened_at).await
}

/// 获取 Entry
/// 
/// @param pool - 数据库连接池
/// @param session - Vault 会话
/// @param id - Entry ID
/// 
/// @return  Entry
pub async fn get(pool: &SqlitePool, session: &VaultSession, id: String) -> Result<EntryDto, AppError> {
  if id.is_empty() {
    return Err(AppError::invalid_argument("id must not be empty"));
  }
  let config = VaultService::load_config(pool).await?;
  require_unlocked(session, config.mode)?;
  EntryService::get(pool, session, &id).await
}
