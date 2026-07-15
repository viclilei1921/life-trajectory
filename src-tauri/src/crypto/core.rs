use argon2::{Argon2, Params};
use chacha20::cipher::{KeyIvInit, StreamCipher, StreamCipherSeek};
use chacha20::ChaCha20;
use rand::Rng;

/// BLOB 魔数标题：告知解码器本段数据使用 ChaCha20 加密。
pub const MAGIC_TITLE: &[u8; 8] = b"chacha20";

/// 版本 1：密码 + blob 内 salt 派生密钥。
pub const VERSION_PASSWORD: u8 = 1;
/// 版本 2：调用方直接提供 32 字节密钥。
pub const VERSION_KEY: u8 = 2;

/// 盐的长度
pub const SALT_LEN: usize = 16;
/// ChaCha20 加密的 Nonce 长度
pub const NONCE_LEN: usize = 12;

/// 生成随机盐
pub fn generate_salt() -> [u8; SALT_LEN] {
  let mut salt = [0u8; SALT_LEN];
  rand::rng().fill_bytes(&mut salt);
  salt
}

/// 生成随机 Nonce
pub fn generate_nonce() -> [u8; NONCE_LEN] {
  let mut nonce = [0u8; NONCE_LEN];
  rand::rng().fill_bytes(&mut nonce);
  nonce
}

/// 派生密钥（Argon2id: m=15MB, t=3, p=1, out=32）
pub fn derive_key(password: &str, salt: &[u8]) -> [u8; 32] {
  let mut key = [0u8; 32];
  let params = Params::new(15000, 3, 1, Some(32)).unwrap_or_default();
  let argon2 = Argon2::new(argon2::Algorithm::Argon2id, argon2::Version::V0x13, params);
  argon2.hash_password_into(password.as_bytes(), salt, &mut key).expect("Failed to derive key");
  key
}

/// 核心处理函数：支持 Seek，且必须传入 offset (偏移量)
pub fn encrypt_decrypt_at_offset(data: &mut [u8], offset: u64, key: &[u8; 32], nonce: &[u8; 12]) {
  let mut cipher = ChaCha20::new(key.into(), nonce.into());
  cipher.seek(offset);
  cipher.apply_keystream(data);
}
