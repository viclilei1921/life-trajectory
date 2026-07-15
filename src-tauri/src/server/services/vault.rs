//! Vault 加密会话与业务编排。
//!
//! # 加密模型（DEK + KEK）
//!
//! ```text
//! 用户密码 ──Argon2──▶ KEK（密钥加密密钥，32 字节）
//!                          │
//!                          ▼ wrap / unwrap
//!                     wrapped_dek（持久化在 vault_config 表）
//!                          │
//!                          ▼ unlock 后驻留内存
//!                        DEK（数据加密密钥，32 字节）
//!                          │
//!                          ▼ encrypt_v2 / decrypt_v2
//!                   entries.content_encrypted
//! ```
//!
//! - **DEK**：加密日记正文；解锁后存在 [`VaultSession`] 内存中，锁定或进程退出即丢失。
//! - **KEK**：仅用于包装/解包 DEK；password 模式下由用户密码 + salt 派生。
//! - **改密码只重包装 DEK**，不必重加密全部 entries。
//!
//! # 两种安全模式
//!
//! | 模式 | 行为 |
//! | --- | --- |
//! | `none` | 明文存储；无 DEK；`is_unlocked` 恒为 true |
//! | `password` | 启动时 locked；用户 unlock 后 DEK 进内存；entry 读写需 `require_dek` |
//!
//! # 职责划分
//!
//! - 本文件：`VaultSession`（内存状态 + DEK 包装原语）+ [`VaultService`]（读写在 DB 的配置，编排用例）
//! - `repositories/vault.rs`：读写 `vault_config` 表
//! - `services/entry.rs`：读写 entry 时调用 `session.require_dek()` 加解密正文
//! - DEK **绝不经 IPC 下发**；前端只知道 `VaultStatusDto { mode, isUnlocked, cryptoVersion }`

use rand::Rng;
use sqlx::SqlitePool;
use zeroize::Zeroizing;

use crate::crypto::blob::{decrypt_v2, encrypt_v2};
use crate::crypto::core::{derive_key, generate_salt, SALT_LEN};
use crate::crypto::error::CryptoError;
use crate::server::dto::VaultStatusDto;
use crate::server::error::{AppError, BizCode};
use crate::server::models::{VaultConfig, VaultMode};
use crate::server::repositories::vault as vault_repo;

const DEK_LEN: usize = 32;

/// 包装明文尾部校验串（可公开）。错密码解出垃圾字节时 magic 对不上 → 拒绝解锁。
/// 布局：`DEK(32) ‖ VERIFY_MAGIC`。
const VERIFY_MAGIC: &[u8] = b"LTVAULT1";
const WRAP_PLAIN_LEN: usize = DEK_LEN + VERIFY_MAGIC.len();

/// 生成随机 DEK；[`Zeroizing`] 保证 Drop 时清零内存。
fn generate_dek() -> Zeroizing<[u8; DEK_LEN]> {
  let mut dek = [0u8; DEK_LEN];
  rand::rng().fill_bytes(&mut dek);
  Zeroizing::new(dek)
}

/// 用 KEK 加密 `DEK ‖ VERIFY_MAGIC`，产出持久化 BLOB（v2 格式，含 nonce）。
fn wrap_dek(dek: &[u8; DEK_LEN], wrapping_key: &[u8; 32]) -> Vec<u8> {
  let mut plain = [0u8; WRAP_PLAIN_LEN];
  plain[..DEK_LEN].copy_from_slice(dek);
  plain[DEK_LEN..].copy_from_slice(VERIFY_MAGIC);
  encrypt_v2(&plain, wrapping_key)
}

/// 用 KEK 解密 wrapped_dek；校验末尾 magic 后还原 DEK。
fn unwrap_dek(wrapped: &[u8], wrapping_key: &[u8; 32]) -> Result<Zeroizing<[u8; DEK_LEN]>, CryptoError> {
  let bytes = decrypt_v2(wrapped, wrapping_key)?;
  if bytes.len() != WRAP_PLAIN_LEN || !bytes[DEK_LEN..].eq(VERIFY_MAGIC) {
    return Err(CryptoError::DecryptFailed);
  }
  let dek: [u8; DEK_LEN] = bytes[..DEK_LEN].try_into().map_err(|_| CryptoError::DecryptFailed)?;
  Ok(Zeroizing::new(dek))
}

/// 将底层加密错误映射为前端可识别的业务码。
fn map_crypto_error(err: CryptoError) -> AppError {
  match err {
    CryptoError::InvalidBlob | CryptoError::UnsupportedVersion => AppError::biz(BizCode::CryptoInvalidBlob),
    // 解包 DEK 失败通常意味着密码错误或密文损坏
    CryptoError::DecryptFailed => AppError::biz(BizCode::VaultInvalidPassword),
  }
}

/// 运行时 Vault 会话，持有内存中的 DEK。
///
/// 实例存放在 [`AppState::vault`](crate::server::state::AppState) 的 `Mutex` 中，
/// 全进程共享一份；Tauri command 通过 `State<AppState>` 读写。
pub struct VaultSession {
  /// `None` = locked；`Some` = unlocked，可加密/解密 entry
  dek: Option<Zeroizing<[u8; DEK_LEN]>>,
  /// 与 `vault_config.crypto_version` 对齐，供未来算法迁移
  crypto_version: i32,
}

impl Default for VaultSession {
  fn default() -> Self {
    Self {
      dek: None,
      crypto_version: 1,
    }
  }
}

impl VaultSession {
  /// 创建空会话（locked）；`crypto_version` 从 DB 配置读取。
  pub fn new(crypto_version: i32) -> Self {
    Self {
      dek: None,
      crypto_version,
    }
  }

  /// 加密版本
  pub fn crypto_version(&self) -> i32 {
    self.crypto_version
  }

  /// 是否已解锁
  pub fn is_unlocked(&self) -> bool {
    self.dek.is_some()
  }

  /// entry 加解密前调用；未解锁返回 `VAULT_LOCKED`。
  pub fn require_dek(&self) -> Result<&[u8; DEK_LEN], AppError> {
    self.dek.as_deref().ok_or_else(|| AppError::biz(BizCode::VaultLocked))
  }

  /// 清除内存 DEK（`Zeroizing` 自动 zeroize）。
  pub fn lock(&mut self) {
    self.dek = None;
  }

  /// 注入 DEK 到内存
  fn inject_dek(&mut self, dek: Zeroizing<[u8; DEK_LEN]>) {
    self.dek = Some(dek);
  }

  /// password 模式解锁：password + salt → KEK → unwrap DEK → 注入内存。
  pub fn unlock_with_password(&mut self, password: &str, salt: &[u8], wrapped_dek: &[u8]) -> Result<(), AppError> {
    if salt.len() != SALT_LEN {
      return Err(AppError::biz(BizCode::VaultInvalidPassword));
    }
    let kek = derive_key(password, salt);
    let dek = unwrap_dek(wrapped_dek, &kek).map_err(map_crypto_error)?;
    self.inject_dek(dek);
    Ok(())
  }

  /// 首次设置主密码：生成 DEK → 用 KEK 包装 → 返回 (已解锁 session, salt, wrapped_dek)。
  ///
  /// 调用方（[`VaultService::setup_password`]）负责将 salt / wrapped_dek 写入 DB。
  pub fn create_password_wrapped(password: &str) -> Result<(Self, [u8; SALT_LEN], Vec<u8>), AppError> {
    let dek = generate_dek();
    let salt = generate_salt();
    let kek = derive_key(password, &salt);
    let wrapped = wrap_dek(&*dek, &kek);
    let mut session = Self::new(1);
    session.inject_dek(dek);
    Ok((session, salt, wrapped))
  }

  /// 改密码：验证旧密码 → 用新 KEK 重包装**同一份 DEK**（entries 无需重加密）。
  pub fn rewrap_with_new_password(
    &self,
    old_password: &str,
    old_salt: &[u8],
    old_wrapped: &[u8],
    new_password: &str,
  ) -> Result<(Vec<u8>, Vec<u8>), AppError> {
    let dek = self.require_dek()?;
    if old_salt.len() != SALT_LEN {
      return Err(AppError::biz(BizCode::VaultInvalidPassword));
    }
    // 用旧密码解包 DB 中的 wrapped_dek，与内存 DEK 比对，防止旧密码错误
    let old_kek = derive_key(old_password, old_salt);
    let verified = unwrap_dek(old_wrapped, &old_kek).map_err(map_crypto_error)?;
    if verified.as_ref() != dek {
      return Err(AppError::biz(BizCode::VaultInvalidPassword));
    }

    let new_salt = generate_salt();
    let new_kek = derive_key(new_password, &new_salt);
    let new_wrapped = wrap_dek(dek, &new_kek);
    Ok((new_salt.to_vec(), new_wrapped))
  }
}

/// Vault 用例编排：连接 DB 配置（[`vault_repo`]）与内存会话（[`VaultSession`]）。
pub struct VaultService;

impl VaultService {
  /// 应用启动时初始化会话。
  ///
  /// - `none` / `password` 模式均创建 **locked** 空 session
  /// - password 模式需前端调用 `unlock` 后才可读写加密 entry
  pub async fn bootstrap_session(pool: &SqlitePool) -> Result<VaultSession, AppError> {
    let config = vault_repo::load(pool).await?;
    Ok(VaultSession::new(config.crypto_version))
  }

  /// 根据 mode 计算前端可见的解锁状态（`none` 模式恒为 unlocked）。
  pub fn status(session: &VaultSession, mode: VaultMode) -> VaultStatusDto {
    let unlocked = match mode {
      VaultMode::None => true,
      VaultMode::Password => session.is_unlocked(),
    };
    VaultStatusDto::new(mode, unlocked, session.crypto_version())
  }

  pub async fn get_status(pool: &SqlitePool, session: &VaultSession) -> Result<VaultStatusDto, AppError> {
    let config = vault_repo::load(pool).await?;
    Ok(Self::status(session, config.mode))
  }

  /// password 模式解锁：校验 mode → 读 DB salt/wrapped_dek → 解包 DEK 进内存。
  pub async fn unlock(
    pool: &SqlitePool,
    session: &mut VaultSession,
    password: &str,
  ) -> Result<VaultStatusDto, AppError> {
    let config = vault_repo::load(pool).await?;
    if config.mode != VaultMode::Password {
      return Err(AppError::biz(BizCode::VaultModeMismatch));
    }
    if session.is_unlocked() {
      return Err(AppError::biz(BizCode::VaultAlreadyUnlocked));
    }
    let (salt, wrapped) = password_wrapped(&config)?;
    session.unlock_with_password(password, salt, wrapped)?;
    Ok(Self::status(session, config.mode))
  }

  /// 锁定：清除内存 DEK；`none` 模式不允许锁定。
  pub async fn lock(pool: &SqlitePool, session: &mut VaultSession) -> Result<VaultStatusDto, AppError> {
    let config = vault_repo::load(pool).await?;
    if config.mode == VaultMode::None {
      return Err(AppError::biz(BizCode::VaultModeMismatch));
    }
    session.lock();
    Ok(Self::status(session, config.mode))
  }

  /// 从 `none` 升级为 `password`：生成 DEK + 包装参数，写 DB，替换全局 session。
  pub async fn setup_password(
    pool: &SqlitePool,
    session_slot: &mut VaultSession,
    password: &str,
  ) -> Result<VaultStatusDto, AppError> {
    let (session, salt, wrapped) = VaultSession::create_password_wrapped(password)?;
    vault_repo::save(pool, VaultMode::Password, Some(&salt), Some(&wrapped), None).await?;
    let status = Self::status(&session, VaultMode::Password);
    *session_slot = session;
    Ok(status)
  }

  /// 改密码：重包装 DEK 并更新 DB 中的 salt / wrapped_dek。
  pub async fn change_password(
    pool: &SqlitePool,
    session: &VaultSession,
    old_password: &str,
    new_password: &str,
  ) -> Result<(), AppError> {
    let config = vault_repo::load(pool).await?;
    let (salt, wrapped) = password_wrapped(&config)?;
    let (new_salt, new_wrapped) = session.rewrap_with_new_password(old_password, salt, wrapped, new_password)?;
    vault_repo::save(pool, VaultMode::Password, Some(&new_salt), Some(&new_wrapped), None).await
  }

  pub async fn load_config(pool: &SqlitePool) -> Result<VaultConfig, AppError> {
    vault_repo::load(pool).await
  }
}

/// 从 DB 配置取出 password 模式所需的 salt 与 wrapped_dek。
fn password_wrapped(config: &VaultConfig) -> Result<(&[u8], &[u8]), AppError> {
  let salt = config
    .kdf_salt
    .as_deref()
    .ok_or_else(|| AppError::internal("missing kdf_salt"))?;
  let wrapped = config
    .wrapped_dek
    .as_deref()
    .ok_or_else(|| AppError::internal("missing wrapped_dek"))?;
  Ok((salt, wrapped))
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn wrapped_dek_roundtrip() {
    let dek = generate_dek();
    let kek = derive_key("test-password", &generate_salt());
    let wrapped = wrap_dek(&*dek, &kek);
    let unwrapped = unwrap_dek(&wrapped, &kek).unwrap();
    assert_eq!(unwrapped.as_ref(), dek.as_ref());
  }

  #[test]
  fn lock_clears_dek() {
    let mut session = VaultSession::default();
    session.inject_dek(generate_dek());
    assert!(session.is_unlocked());
    session.lock();
    assert!(!session.is_unlocked());
    assert!(session.require_dek().is_err());
  }
}
