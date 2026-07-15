use log::error;

use super::error::{AppError, BizCode};

/// 将 AppError 编码为 Tauri 命令错误字符串。
///
/// - 业务码原样返回
/// - `INVALID_ARGUMENT` 保留前缀与说明（供前端展示参数问题）
/// - `INTERNAL_ERROR` 仅返回稳定码，完整细节写入日志
pub fn encode_error(err: AppError) -> String {
  match err.code() {
    BizCode::Internal => {
      error!("internal error: {err}");
      BizCode::Internal.as_str().to_string()
    }
    BizCode::InvalidArgument => match err {
      AppError::Message(s) => s,
      AppError::Biz(code) => code.as_str().to_string(),
    },
    code => code.as_str().to_string(),
  }
}

/// 将 Result 映射为 Tauri 命令 Result
pub fn to_command_result<T>(result: Result<T, AppError>) -> Result<T, String> {
  result.map_err(encode_error)
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::server::error::AppError;

  #[test]
  fn internal_error_strips_details() {
    let err = AppError::internal("permission denied");
    assert_eq!(encode_error(err), "INTERNAL_ERROR");
  }

  #[test]
  fn invalid_argument_keeps_message() {
    let err = AppError::invalid_argument("password is empty");
    assert_eq!(encode_error(err), "INVALID_ARGUMENT: password is empty");
  }

  #[test]
  fn biz_code_passes_through() {
    let err = AppError::biz(BizCode::VaultLocked);
    assert_eq!(encode_error(err), "VAULT_LOCKED");
  }
}
