//! Entry 数据访问：纯 SQLx 读写 `entries` 表，不含业务与加解密逻辑。

use sqlx::{Row, SqlitePool};

use crate::server::error::{AppError, BizCode};
use crate::server::models::{Entry, NewEntry};

/// 插入新条目（明文 `content` 与加密 `content_encrypted` 由 service 层决定）。
pub async fn insert(pool: &SqlitePool, entry: &NewEntry) -> Result<(), AppError> {
  sqlx::query(
    "INSERT INTO entries (id, title, content, content_encrypted, is_encrypted, happened_at, created_at, updated_at)
     VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
  )
  .bind(&entry.id)
  .bind(&entry.title)
  .bind(&entry.content)
  .bind(&entry.content_encrypted)
  .bind(if entry.is_encrypted { 1 } else { 0 })
  .bind(entry.happened_at)
  .bind(entry.created_at)
  .bind(entry.updated_at)
  .execute(pool)
  .await?;
  Ok(())
}

/// 按 id 查询未软删条目；不存在时返回 `EntryNotFound`。
pub async fn find_by_id(pool: &SqlitePool, id: &str) -> Result<Entry, AppError> {
  let row = sqlx::query(
    "SELECT id, title, content, content_encrypted, is_encrypted, happened_at
     FROM entries WHERE id = ? AND deleted_at IS NULL",
  )
  .bind(id)
  .fetch_optional(pool)
  .await?
  .ok_or_else(|| AppError::biz(BizCode::EntryNotFound))?;

  let is_encrypted: i32 = row.get("is_encrypted");
  Ok(Entry {
    id: row.get("id"),
    title: row.get("title"),
    content: row.get("content"),
    content_encrypted: row.get("content_encrypted"),
    is_encrypted: is_encrypted == 1,
    happened_at: row.get("happened_at"),
  })
}
