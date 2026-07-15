//! 内部统一用 [`AppError`]，command 边界经 [`transport::to_command_result`](crate::server::transport::to_command_result) 转为 `String` 传给前端。
//!
//! - [`BizCode`]：稳定业务码（如 `VAULT_LOCKED`），供前端分支判断
//! - [`AppError::Biz`]：纯业务失败，对外只返回码
//! - [`AppError::Message`]：带上下文（参数校验、内部异常）；内部细节经 [`transport::encode_error`](crate::server::transport::encode_error) 脱敏后返回前端
//!
//! 底层错误通过 `From` 自动映射：`CryptoError` → 业务码，`sqlx::Error` → `internal(...)`。

use thiserror::Error;

use crate::crypto::error::CryptoError;

/// 业务错误码（对外稳定字符串）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BizCode {
  /// Vault 被锁定
  VaultLocked,
  /// Vault 已经解锁
  VaultAlreadyUnlocked,
  /// Vault 密码无效
  VaultInvalidPassword,
  /// Vault 模式不匹配
  VaultModeMismatch,
  /// 迁移进行中
  MigrationInProgress,
  /// 条目未找到
  EntryNotFound,
  /// 加密无效的 blob
  CryptoInvalidBlob,
  /// 无效的参数
  InvalidArgument,
  /// 内部错误
  Internal,
}

impl BizCode {
  pub fn as_str(self) -> &'static str {
    match self {
      BizCode::VaultLocked => "VAULT_LOCKED",
      BizCode::VaultAlreadyUnlocked => "VAULT_ALREADY_UNLOCKED",
      BizCode::VaultInvalidPassword => "VAULT_INVALID_PASSWORD",
      BizCode::VaultModeMismatch => "VAULT_MODE_MISMATCH",
      BizCode::MigrationInProgress => "MIGRATION_IN_PROGRESS",
      BizCode::EntryNotFound => "ENTRY_NOT_FOUND",
      BizCode::CryptoInvalidBlob => "CRYPTO_INVALID_BLOB",
      BizCode::InvalidArgument => "INVALID_ARGUMENT",
      BizCode::Internal => "INTERNAL_ERROR",
    }
  }
}

/// 实现 Display 让错误信息更友好
impl std::fmt::Display for BizCode {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(self.as_str())
  }
}

/// 内部统一错误类型
#[derive(Debug, Error)]
pub enum AppError {
  /// 前端只关心「发生了什么」，不需要细节
  #[error("{0}")]
  Biz(BizCode),
  /// 需要上下文，比如「哪个参数错了」「DB 报了什么错」
  #[error("{0}")]
  Message(String),
}

/// 创建业务错误——怎么创建错误
impl AppError {
  /// 创建业务错误
  pub fn biz(code: BizCode) -> Self {
    AppError::Biz(code)
  }

  /// 创建消息错误
  pub fn invalid_argument(msg: impl Into<String>) -> Self {
    AppError::Message(format!("{}: {}", BizCode::InvalidArgument.as_str(), msg.into()))
  }

  /// 创建内部错误
  pub fn internal(msg: impl Into<String>) -> Self {
    AppError::Message(format!("{}: {}", BizCode::Internal.as_str(), msg.into()))
  }

  /// 获取错误码
  pub fn code(&self) -> BizCode {
    match self {
      AppError::Biz(code) => *code,
      AppError::Message(s) if s.starts_with("INVALID_ARGUMENT") => BizCode::InvalidArgument,
      AppError::Message(_) => BizCode::Internal,
    }
  }
}

/// 不同来源的错误自动变成 AppError
/// 从 BizCode 创建 AppError
impl From<BizCode> for AppError {
  fn from(code: BizCode) -> Self {
    AppError::Biz(code)
  }
}

/// 从 CryptoError 创建 AppError
impl From<CryptoError> for AppError {
  fn from(value: CryptoError) -> Self {
    match value {
      CryptoError::InvalidBlob | CryptoError::UnsupportedVersion => AppError::biz(BizCode::CryptoInvalidBlob),
      CryptoError::DecryptFailed => AppError::biz(BizCode::VaultInvalidPassword),
    }
  }
}

/// 从 sqlx::Error 创建 AppError
impl From<sqlx::Error> for AppError {
  fn from(value: sqlx::Error) -> Self {
    AppError::internal(value.to_string())
  }
}

/// 从 AppError 创建 String
impl From<AppError> for String {
  fn from(value: AppError) -> Self {
    match value {
      AppError::Biz(code) => code.as_str().to_string(),
      AppError::Message(msg) => msg,
    }
  }
}
