const ERROR_MESSAGES: Record<string, string> = {
  VAULT_LOCKED: '保险库已锁定，请先解锁',
  VAULT_ALREADY_UNLOCKED: '保险库已经处于解锁状态',
  VAULT_INVALID_PASSWORD: '密码错误，请重试',
  VAULT_MODE_MISMATCH: '当前操作与保险库模式不匹配',
  MIGRATION_IN_PROGRESS: '数据迁移进行中，请稍后再试',
  ENTRY_NOT_FOUND: '找不到该记录',
  CRYPTO_INVALID_BLOB: '数据损坏或加密版本不支持',
  INTERNAL_ERROR: '内部错误，请稍后重试'
};

/**
 * 将 Tauri invoke 错误转为用户可读的中文提示
 */
export function formatAppError(error: unknown): string {
  const message = error instanceof Error ? error.message : String(error);

  if (message.startsWith('INVALID_ARGUMENT:')) {
    return message.replace('INVALID_ARGUMENT:', '参数错误：').trim();
  }

  return ERROR_MESSAGES[message] ?? message;
}
