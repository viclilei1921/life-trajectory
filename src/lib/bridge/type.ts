/**
 * @description Vault 状态
 * @property mode - Vault 模式
 * @property isUnlocked - Vault 是否解锁
 * @property cryptoVersion - Vault 加密版本
 */
export interface VaultStatus {
  mode: 'none' | 'password';
  isUnlocked: boolean;
  cryptoVersion: number;
}

/**
 * @description 创建 Entry 参数
 * @property title - Entry 标题
 * @property content - Entry 内容
 * @property happened_at - Entry 发生时间
 */
export interface CreateEntryArgs {
  title?: string;
  content: string;
  happenedAt: number;
}

/**
 * @description 获取 Entry 响应
 * @property id - Entry ID
 * @property title - Entry 标题
 * @property content - Entry 内容
 * @property happenedAt - Entry 发生时间
 * @property isEncrypted - Entry 是否加密
 */
export interface EntryResponse {
  id: string;
  title: string | null;
  content: string;
  happenedAt: number;
  isEncrypted: boolean;
}

/**
 * @description 列表摘要（与未来 list IPC 对齐；UI 先行阶段由占位实现返回）
 */
export interface EntrySummary {
  id: string;
  title: string | null;
  /** 正文摘要，列表展示用 */
  excerpt: string;
  happenedAt: number;
  isEncrypted: boolean;
}
