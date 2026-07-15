//! Vault 配置数据访问：读写 `vault_config` 单行表（`id = 1`）。

use sqlx::{Row, SqlitePool};

use crate::server::error::AppError;
use crate::server::models::{VaultConfig, VaultMode};

/// 读取全局 Vault 配置（迁移保证 id=1 行始终存在）。
pub async fn load(pool: &SqlitePool) -> Result<VaultConfig, AppError> {
  let row = sqlx::query(
    "SELECT mode, kdf_salt, wrapped_dek, wrap_nonce, crypto_version FROM vault_config WHERE id = 1",
  )
  .fetch_one(pool)
  .await?;

  let mode_str: String = row.get("mode");
  Ok(VaultConfig {
    mode: VaultMode::parse(&mode_str)?,
    kdf_salt: row.get("kdf_salt"),
    wrapped_dek: row.get("wrapped_dek"),
    wrap_nonce: row.get("wrap_nonce"),
    crypto_version: row.get("crypto_version"),
  })
}

/// 更新 Vault 模式与 DEK 包装参数（setup / unlock 迁移 / 改密后调用）。
pub async fn save(
  pool: &SqlitePool,
  mode: VaultMode,
  kdf_salt: Option<&[u8]>,
  wrapped_dek: Option<&[u8]>,
  wrap_nonce: Option<&[u8]>,
) -> Result<(), AppError> {
  let now = unix_now();
  sqlx::query(
    "UPDATE vault_config
     SET mode = ?, kdf_salt = ?, wrapped_dek = ?, wrap_nonce = ?, updated_at = ?
     WHERE id = 1",
  )
  .bind(mode.as_str())
  .bind(kdf_salt)
  .bind(wrapped_dek)
  .bind(wrap_nonce)
  .bind(now)
  .execute(pool)
  .await?;
  Ok(())
}

fn unix_now() -> i64 {
  std::time::SystemTime::now()
    .duration_since(std::time::UNIX_EPOCH)
    .map(|d| d.as_secs() as i64)
    .unwrap_or(0)
}
