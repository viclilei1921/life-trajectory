//! Entry 对外传输结构（Tauri command → 前端 JSON）。
//!
//! 与 [`Entry`](crate::server::models::Entry) 的区别：DB 层可能存密文 blob，
//! DTO 的 `content` 始终是解密后的明文，供 UI 直接展示。

use serde::Serialize;

use crate::server::models::Entry;

/// 条目响应体，序列化为 camelCase JSON（如 `happenedAt`）。
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EntryDto {
  pub id: String,
  pub title: Option<String>,
  /// 明文内容（读取时已解密；创建时直接回传用户输入）
  pub content: String,
  /// Unix 时间戳（秒）
  pub happened_at: i64,
  /// 是否在 DB 中以加密 blob 存储
  pub is_encrypted: bool,
}

impl EntryDto {
  /// 从 DB 实体组装：传入已解密的 `plaintext`，其余字段取自 `entry`。
  pub fn from_entry(entry: Entry, plaintext: String) -> Self {
    Self {
      id: entry.id,
      title: entry.title,
      content: plaintext,
      happened_at: entry.happened_at,
      is_encrypted: entry.is_encrypted,
    }
  }

  /// 创建成功后直接构造，无需再查库（content 用用户提交的明文）。
  pub fn created(id: String, title: Option<String>, content: String, happened_at: i64, is_encrypted: bool) -> Self {
    Self {
      id,
      title,
      content,
      happened_at,
      is_encrypted,
    }
  }
}
