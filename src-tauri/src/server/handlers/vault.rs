//! Vault 用例入口：入参校验 + Vault 解锁门禁 + 调用 [`VaultService`]。

use sqlx::SqlitePool;

use crate::server::dto::VaultStatusDto;
use crate::server::error::{AppError, BizCode};
use crate::server::middleware::require_unlocked;
use crate::server::models::VaultMode;
use crate::server::services::VaultService;
use crate::server::services::vault::VaultSession;

/// 获取 Vault 状态
/// 
/// @param pool - 数据库连接池
/// @param session - Vault 会话
/// 
/// @return  Vault 状态
pub async fn get_status(pool: &SqlitePool, session: &VaultSession) -> Result<VaultStatusDto, AppError> {
  VaultService::get_status(pool, session).await
}

/// 解锁 Vault
/// 
/// @param pool - 数据库连接池
/// @param session - Vault 会话
/// @param password - 密码
/// 
/// @return  Vault 状态
pub async fn unlock(
  pool: &SqlitePool,
  session: &mut VaultSession,
  password: String,
) -> Result<VaultStatusDto, AppError> {
  if password.is_empty() {
    return Err(AppError::invalid_argument("password must not be empty"));
  }
  VaultService::unlock(pool, session, &password).await
}

/// 锁定 Vault
/// 
/// @param pool - 数据库连接池
/// @param session - Vault 会话
/// 
/// @return  Vault 状态
pub async fn lock(pool: &SqlitePool, session: &mut VaultSession) -> Result<VaultStatusDto, AppError> {
  VaultService::lock(pool, session).await
}

/// 设置 Vault 密码
/// 
/// @param pool - 数据库连接池
/// @param session - Vault 会话
/// @param password - 密码
/// 
/// @return  Vault 状态
pub async fn setup_password(
  pool: &SqlitePool,
  session: &mut VaultSession,
  password: String,
) -> Result<VaultStatusDto, AppError> {
  if password.is_empty() {
    return Err(AppError::invalid_argument("password must not be empty"));
  }
  VaultService::setup_password(pool, session, &password).await
}

/// 修改 Vault 密码
/// 
/// @param pool - 数据库连接池
/// @param session - Vault 会话
/// @param old - 旧密码
/// @param new - 新密码
/// 
/// @return 结果
pub async fn change_password(
  pool: &SqlitePool,
  session: &VaultSession,
  old: String,
  new: String,
) -> Result<(), AppError> {
  if old.is_empty() || new.is_empty() {
    return Err(AppError::invalid_argument("password must not be empty"));
  }
  let config = VaultService::load_config(pool).await?;
  if config.mode != VaultMode::Password {
    return Err(AppError::biz(BizCode::VaultModeMismatch));
  }
  require_unlocked(session, config.mode)?;
  VaultService::change_password(pool, session, &old, &new).await
}
