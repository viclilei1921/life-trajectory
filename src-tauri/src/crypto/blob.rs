use super::core::*;
use super::error::CryptoError;

/// 版本 1 头部长度：MAGIC_TITLE + VERSION + SALT + NONCE
pub const V1_HEADER_LEN: usize = MAGIC_TITLE.len() + 1 + SALT_LEN + NONCE_LEN;
/// 版本 2 头部长度：MAGIC_TITLE + VERSION + NONCE
pub const V2_HEADER_LEN: usize = MAGIC_TITLE.len() + 1 + NONCE_LEN;

/// 版本 1 加密：MAGIC_TITLE + VERSION_PASSWORD + SALT + NONCE + ciphertext
pub fn encrypt_v1(plaintext: &[u8], password: &str) -> Vec<u8> {
  let salt = generate_salt();
  let nonce = generate_nonce();
  let key = derive_key(password, &salt);

  let mut ciphertext = plaintext.to_vec();
  encrypt_decrypt_at_offset(&mut ciphertext, 0, &key, &nonce);

  let mut out = Vec::with_capacity(V1_HEADER_LEN + ciphertext.len());
  out.extend_from_slice(MAGIC_TITLE);
  out.push(VERSION_PASSWORD);
  out.extend_from_slice(&salt);
  out.extend_from_slice(&nonce);
  out.extend_from_slice(&ciphertext);
  out
}

/// 版本 1 解密
pub fn decrypt_v1(blob: &[u8], password: &str) -> Result<Vec<u8>, CryptoError> {
  if blob.len() < V1_HEADER_LEN {
    return Err(CryptoError::InvalidBlob);
  }
  if &blob[..MAGIC_TITLE.len()] != MAGIC_TITLE {
    return Err(CryptoError::InvalidBlob);
  }
  if blob[MAGIC_TITLE.len()] != VERSION_PASSWORD {
    return Err(CryptoError::UnsupportedVersion);
  }

  let salt_start = MAGIC_TITLE.len() + 1;
  let salt: [u8; SALT_LEN] = blob[salt_start..salt_start + SALT_LEN]
    .try_into()
    .map_err(|_| CryptoError::InvalidBlob)?;
  let nonce_start = salt_start + SALT_LEN;
  let nonce: [u8; NONCE_LEN] = blob[nonce_start..nonce_start + NONCE_LEN]
    .try_into()
    .map_err(|_| CryptoError::InvalidBlob)?;

  let key = derive_key(password, &salt);
  let mut plaintext = blob[V1_HEADER_LEN..].to_vec();
  encrypt_decrypt_at_offset(&mut plaintext, 0, &key, &nonce);
  Ok(plaintext)
}

/// 版本 2 加密：MAGIC_TITLE + VERSION_KEY + NONCE + ciphertext
pub fn encrypt_v2(plaintext: &[u8], key: &[u8; 32]) -> Vec<u8> {
  let nonce = generate_nonce();
  let mut ciphertext = plaintext.to_vec();
  encrypt_decrypt_at_offset(&mut ciphertext, 0, key, &nonce);

  let mut out = Vec::with_capacity(V2_HEADER_LEN + ciphertext.len());
  out.extend_from_slice(MAGIC_TITLE);
  out.push(VERSION_KEY);
  out.extend_from_slice(&nonce);
  out.extend_from_slice(&ciphertext);
  out
}

/// 版本 2 解密
pub fn decrypt_v2(blob: &[u8], key: &[u8; 32]) -> Result<Vec<u8>, CryptoError> {
  if blob.len() < V2_HEADER_LEN {
    return Err(CryptoError::InvalidBlob);
  }
  if &blob[..MAGIC_TITLE.len()] != MAGIC_TITLE {
    return Err(CryptoError::InvalidBlob);
  }
  if blob[MAGIC_TITLE.len()] != VERSION_KEY {
    return Err(CryptoError::UnsupportedVersion);
  }

  let nonce_start = MAGIC_TITLE.len() + 1;
  let nonce: [u8; NONCE_LEN] = blob[nonce_start..nonce_start + NONCE_LEN]
    .try_into()
    .map_err(|_| CryptoError::InvalidBlob)?;

  let mut plaintext = blob[V2_HEADER_LEN..].to_vec();
  encrypt_decrypt_at_offset(&mut plaintext, 0, key, &nonce);
  Ok(plaintext)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn v1_roundtrip() {
    let plain = b"password encrypted";
    let blob = encrypt_v1(plain, "secret");
    let out = decrypt_v1(&blob, "secret").unwrap();
    assert_eq!(out, plain);
  }

  #[test]
  fn v2_roundtrip() {
    let key = derive_key("dek-seed", &generate_salt());
    let plain = b"key encrypted";
    let blob = encrypt_v2(plain, &key);
    let out = decrypt_v2(&blob, &key).unwrap();
    assert_eq!(out, plain);
  }

  #[test]
  fn v2_nonce_unique() {
    let key = derive_key("dek-seed", &generate_salt());
    let plain = b"same text";
    let a = encrypt_v2(plain, &key);
    let b = encrypt_v2(plain, &key);
    assert_ne!(a, b);
  }
}
