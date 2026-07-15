use thiserror::Error;

/// 加密错误枚举，用于表示加密过程中发生的错误
#[derive(Debug, Error)]
pub enum CryptoError {
  /// 无效的 blob
  #[error("CRYPTO_INVALID_BLOB")]
  InvalidBlob,
  /// 不支持的版本
  #[error("CRYPTO_UNSUPPORTED_VERSION")]
  UnsupportedVersion,
  /// 解密失败
  #[error("CRYPTO_DECRYPT_FAILED")]
  DecryptFailed,
}
