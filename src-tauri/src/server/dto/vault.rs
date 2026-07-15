//! Vault 对外传输结构（Tauri command → 前端 JSON）。

use serde::Serialize;

use crate::server::models::VaultMode;

/// Vault 状态响应体，用于 get_status / unlock / lock 等接口的统一返回。
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VaultStatusDto {
  /// 加密模式：`none` | `password`
  pub mode: String,
  /// DEK 是否已在内存中解锁（`none` 模式恒为 true）
  pub is_unlocked: bool,
  /// 当前 crypto 协议版本，用于未来迁移
  pub crypto_version: i32,
}

impl VaultStatusDto {
  /// 从领域枚举 [`VaultMode`] 转为前端可读字符串。
  pub fn new(mode: VaultMode, is_unlocked: bool, crypto_version: i32) -> Self {
    Self { mode: mode.as_str().to_string(), is_unlocked, crypto_version }
  }
}
