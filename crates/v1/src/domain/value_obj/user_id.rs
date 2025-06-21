use crate::interfaces::http::error::{AppError, AppResult};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UserId(i64);

impl UserId {
  const TARGET: &str = "ユーザーID(user_id)";

  /// user_idが正の整数であることを保証する。
  /// - user_idは本システムにおいて必須の値の為，存在は保証されている。
  pub fn new(user_id: i64) -> AppResult<Self> {
    if user_id <= 0 {
      return Err(AppError::InternalServerError(Some(
        format!("{}は正の整数でなければなりません。", Self::TARGET).into(),
      )));
    }
    Ok(Self(user_id))
  }

  /// user_idの実態(i64)を返す。
  pub fn as_i64(self) -> i64 {
    self.0
  }
}

#[cfg(test)]
mod user_id_test {
  use super::*;
  #[test]
  fn user_id_ok() {
    assert_eq!(UserId::new(1).unwrap().as_i64(), 1);
  }
  #[test]
  fn user_id_0_err() {
    assert!(UserId::new(0).is_err())
  }
  #[test]
  fn user_id_str_minus() {
    assert!(UserId::new(-10).is_err())
  }
}
