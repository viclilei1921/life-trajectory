//! Entry 业务流程编排：按 Vault 模式决定明文/加密存储，读写 [`EntryDto`]。

use sqlx::SqlitePool;

use crate::crypto::blob::{decrypt_v2, encrypt_v2};
use crate::server::dto::EntryDto;
use crate::server::error::AppError;
use crate::server::models::{new_id, NewEntry, VaultMode};
use crate::server::repositories::entry as entry_repo;
use crate::server::repositories::vault as vault_repo;
use crate::server::services::vault::VaultSession;

fn unix_now() -> i64 {
  std::time::SystemTime::now()
    .duration_since(std::time::UNIX_EPOCH)
    .map(|d| d.as_secs() as i64)
    .unwrap_or(0)
}

pub struct EntryService;

impl EntryService {
  /// 创建 Entry
  /// 
  /// @param pool - 数据库连接池
  /// @param session - Vault 会话
  /// @param title - Entry 标题
  /// @param content - Entry 内容
  /// @param happened_at - Entry 发生时间
  /// 
  /// @return  Entry
  pub async fn create(
    pool: &SqlitePool,
    session: &VaultSession,
    title: Option<String>,
    content: String,
    happened_at: i64,
  ) -> Result<EntryDto, AppError> {
    let id = new_id();
    let config = vault_repo::load(pool).await?;
    let now = unix_now();

    let (stored_content, content_encrypted, is_encrypted) = match config.mode {
      VaultMode::None => (content.clone(), None, false),
      VaultMode::Password => {
        let dek = session.require_dek()?;
        let blob = encrypt_v2(content.as_bytes(), dek);
        (String::new(), Some(blob), true)
      }
    };

    entry_repo::insert(
      pool,
      &NewEntry {
        id: id.clone(),
        title: title.clone(),
        content: stored_content,
        content_encrypted,
        is_encrypted,
        happened_at,
        created_at: now,
        updated_at: now,
      },
    )
    .await?;

    Ok(EntryDto::created(id, title, content, happened_at, is_encrypted))
  }

  /// 获取 Entry
  /// 
  /// @param pool - 数据库连接池
  /// @param session - Vault 会话
  /// @param id - Entry ID
  /// 
  /// @return  Entry
  pub async fn get(pool: &SqlitePool, session: &VaultSession, id: &str) -> Result<EntryDto, AppError> {
    let entry = entry_repo::find_by_id(pool, id).await?;
    let plaintext = if entry.is_encrypted {
      let dek = session.require_dek()?;
      let blob = entry
        .content_encrypted
        .as_deref()
        .ok_or_else(|| AppError::internal("missing content_encrypted"))?;
      let bytes = decrypt_v2(blob, dek)?;
      String::from_utf8(bytes).map_err(|e| AppError::internal(e.to_string()))?
    } else {
      entry.content.clone()
    };
    Ok(EntryDto::from_entry(entry, plaintext))
  }
}
