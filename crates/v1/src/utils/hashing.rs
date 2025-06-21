//! ハッシュ化・ハッシュ値検証を行う

use crate::interfaces::http::error::{AppError, AppResult};
use argon2::{
  Algorithm, Argon2, Params, PasswordHasher, PasswordVerifier, Version,
  password_hash::{self, PasswordHash, SaltString, rand_core::OsRng},
};

fn argon2_config() -> Argon2<'static> {
  let params = Params::new(19456, 3, 1, None).expect("Argon2のconfig作成に失敗。");
  Argon2::new(Algorithm::Argon2id, Version::V0x13, params)
}

/// 平文文字列をArgon2でハッシュ化して返す。
pub fn hashing(plain: &str) -> AppResult<String> {
  let salt = SaltString::generate(&mut OsRng);
  let hash = argon2_config()
    .hash_password(plain.as_bytes(), &salt)
    .map_err(|e| AppError::InternalServerError(format!("Hashing failed: {e}").into()))?;
  Ok(hash.to_string())
}

/// 平文文字列とハッシュ文字列を検証する。
pub fn verify_hashed(plain: &str, hashed: &str) -> AppResult<()> {
  let parsed = PasswordHash::new(hashed)
    .map_err(|e| AppError::UnprocessableContent(Some(format!("ハッシュ文字列が不正です: {e}"))))?;

  // 検証
  match argon2_config().verify_password(plain.as_bytes(), &parsed) {
    Ok(_) => Ok(()),
    Err(password_hash::Error::Password) => Err(AppError::Unauthorized(Some(
      "パスワードが一致しません。".into(),
    ))),
    Err(e) => Err(AppError::InternalServerError(Some(format!(
      "Hash verify error: {e}"
    )))),
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn hashing_and_verify_ok() {
    let hash = hashing("secret").unwrap();
    assert!(verify_hashed("secret", &hash).is_ok());
  }

  #[test]
  fn verify_mismatch_err() {
    let hash = hashing("secret").unwrap();
    assert!(verify_hashed("wrong", &hash).is_err());
  }
}
