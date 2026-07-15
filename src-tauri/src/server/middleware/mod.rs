//! 横切校验，由 handlers / services 显式调用。

use crate::server::error::{AppError, BizCode};
use crate::server::models::VaultMode;
use crate::server::services::vault::VaultSession;

/// 要求会话已解锁（`password` 模式；`none` 模式直接通过）。
/// 
/// @param session - Vault 会话
/// @param mode - Vault 模式
/// 
/// @return 结果
pub fn require_unlocked(session: &VaultSession, mode: VaultMode) -> Result<(), AppError> {
  match mode {
    VaultMode::None => Ok(()),
    VaultMode::Password => {
      if session.is_unlocked() {
        Ok(())
      } else {
        Err(AppError::biz(BizCode::VaultLocked))
      }
    }
  }
}
