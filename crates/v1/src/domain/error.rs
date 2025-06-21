//! ドメイン＆インフラ層共通エラー

use argon2::password_hash::Error as Argon2Error;
use sqlx::Error as SqlxError;
use thiserror::Error;

/// SQLxで発生しうるエラー
#[derive(Debug, Error)]
pub enum DatabaseError {
  #[error("Row not found")]
  NotFound,
  #[error(transparent)]
  Sqlx(#[from] SqlxError),
}

/// Argon2で発生しるエラー
#[derive(Debug, Error)]
pub enum HashingError {
  #[error("Password mismatch")]
  PasswordMismatch,
  #[error("Argon2 error: {0}")]
  Argon2(#[from] Argon2Error),
}
