use tokio::sync::Mutex;

use sqlx::SqlitePool;

use crate::server::services::vault::VaultSession;

/// 应用状态，包含 DB 连接池和 Vault 会话
pub struct AppState {
  /// DB 连接池
  pub db: SqlitePool,
  /// Vault 会话
  pub vault: Mutex<VaultSession>,
}
