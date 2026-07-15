//! Vault 领域模型：加密模式、DB 配置与辅助校验。

use serde::{Deserialize, Serialize};

use crate::server::error::{AppError, BizCode};

/// 加密模式
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum VaultMode {
  /// 无加密模式
  None,
  /// 密码加密模式
  Password,
}

impl VaultMode {
  /// 转换为字符串
  pub fn as_str(self) -> &'static str {
    match self {
      VaultMode::None => "none",
      VaultMode::Password => "password",
    }
  }

  /// 解析字符串
  pub fn parse(value: &str) -> Result<Self, AppError> {
    match value {
      "none" => Ok(VaultMode::None),
      "password" => Ok(VaultMode::Password),
      "device" => Err(AppError::internal("vault mode 'device' is no longer supported")),
      _ => Err(AppError::internal(format!("unknown vault mode: {value}"))),
    }
  }
}

/// Vault 配置
#[derive(Debug, Clone)]
pub struct VaultConfig {
  pub mode: VaultMode,
  /// KDF 盐
  pub kdf_salt: Option<Vec<u8>>,
  /// 包装的 DEK
  pub wrapped_dek: Option<Vec<u8>>,
  /// 包装 nonce
  pub wrap_nonce: Option<Vec<u8>>,
  /// 加密版本
  pub crypto_version: i32,
}

impl VaultConfig {
  /// 是否需要解锁
  pub fn requires_unlock(&self) -> bool {
    matches!(self.mode, VaultMode::Password)
  }
}

/// 包装的 DEK 更新
#[derive(Debug, Clone)]
pub struct WrappedDekUpdate {
  /// KDF 盐
  pub kdf_salt: Vec<u8>,
  /// 包装的 DEK
  pub wrapped_dek: Vec<u8>,
  /// 包装 nonce
  pub wrap_nonce: Vec<u8>,
}

/// 确保模式匹配
pub fn ensure_mode(actual: VaultMode, expected: VaultMode) -> Result<(), AppError> {
  if actual != expected {
    return Err(AppError::biz(BizCode::VaultModeMismatch));
  }
  Ok(())
}
