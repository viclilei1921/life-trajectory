//! Entry 领域实体，对应 `entries` 表核心字段（repository 读写用）。

/// 生成标准 UUID v4 主键；仅 Rust 侧分配，不接受上层传入。
pub fn new_id() -> String {
  uuid::Uuid::new_v4().to_string()
}

/// 从 DB 读出的条目；`content` 与 `content_encrypted` 二选一有效。
#[derive(Debug, Clone)]
pub struct Entry {
  pub id: String,
  pub title: Option<String>,
  /// 明文内容（`is_encrypted = false` 时使用）
  pub content: String,
  /// 加密 blob（`is_encrypted = true` 时使用）
  pub content_encrypted: Option<Vec<u8>>,
  /// 是否加密
  pub is_encrypted: bool,
  /// Unix 时间戳（秒）
  pub happened_at: i64,
}

/// 写入 DB 的新条目，含审计时间戳。
#[derive(Debug, Clone)]
pub struct NewEntry {
  /// 主键
  pub id: String,
  /// 标题
  pub title: Option<String>,
  /// 明文内容
  pub content: String,
  /// 加密 blob
  pub content_encrypted: Option<Vec<u8>>,
  /// 是否加密
  pub is_encrypted: bool,
  /// Unix 时间戳（秒）
  pub happened_at: i64,
  /// 创建时间
  pub created_at: i64,
  /// 更新时间
  pub updated_at: i64,
}
