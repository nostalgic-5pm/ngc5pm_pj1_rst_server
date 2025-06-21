use crate::{
  interfaces::http::error::{AppError, AppResult},
  utils::{
    hashing::{hashing, verify_hashed},
    string::is_forbidden_char,
  },
};
use chrono::NaiveDate;
use zeroize::Zeroize;
use zxcvbn::{Score, zxcvbn};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserPassword {
  /// Argon2でハッシュ化されたパスワード
  hash: String,
}

impl UserPassword {
  const TARGET: &str = "パスワード(user_password)";
  const MIN_LEN: usize = 8;
  const MAX_LEN: usize = 64;
  const MIN_ZXCVBN_SCORE: Score = Score::Three;

  /// 平文パスワードの入力を検証し，UserPassword型のオブジェクトを生成する。
  pub fn new<S: AsRef<str>>(
    input: S,
    required: bool,
    user_name: S,
    birth_date: Option<NaiveDate>,
  ) -> AppResult<Option<Self>> {
    // 正規化・必須長さチェック
    // 正規化：先頭末尾の空白をトリムするのみ
    let mut plain = input.as_ref().trim().to_owned();

    // 入力が空文字列の場合，かつrequiredがfalseならNoneを返す。
    if plain.is_empty() && !required {
      return Ok(None);
    }

    // 長さチェック
    if plain.len() < Self::MIN_LEN || plain.len() > Self::MAX_LEN {
      plain.zeroize();
      return Err(AppError::UnprocessableContent(Some(format!(
        "{}は{}文字以上、{}文字以下でなければなりません。",
        Self::TARGET,
        Self::MIN_LEN,
        Self::MAX_LEN
      ))));
    }

    // 使用文字チェック
    if plain.chars().any(is_forbidden_char) {
      plain.zeroize();
      return Err(AppError::UnprocessableContent(Some(format!(
        "{}には使用できない文字が含まれています。",
        Self::TARGET
      ))));
    }

    // ユーザー名と誕生日がパスワードに含まれているかチェック
    let lower_password = plain.to_lowercase();
    let lower_user_name = user_name.as_ref().to_lowercase();
    if lower_password.contains(&lower_user_name) {
      plain.zeroize();
      return Err(AppError::UnprocessableContent(Some(format!(
        "{}にはユーザー名を含めることができません。",
        Self::TARGET
      ))));
    }

    if let Some(birth_date) = birth_date {
      let ymd = birth_date.format("%Y%m%d").to_string();
      let md = birth_date.format("%m%d").to_string();
      if lower_password.contains(&ymd) || lower_password.contains(&md) {
        plain.zeroize();
        return Err(AppError::UnprocessableContent(Some(format!(
          "{}には誕生日を含めることができません。",
          Self::TARGET
        ))));
      }
    }

    // パスワードの強度チェック
    if zxcvbn(&plain, &[&lower_user_name]).score() < Self::MIN_ZXCVBN_SCORE {
      plain.zeroize();
      return Err(AppError::UnprocessableContent(Some(format!(
        "{}は強度が不十分です。より強力なパスワードを使用してください。",
        Self::TARGET
      ))));
    }

    // パスワードをハッシュ化する
    let hash = hashing(&plain)?;

    plain.zeroize();

    // 正常時はUserPassword型のオブジェクトを返す
    Ok(Some(Self { hash }))
  }

  /// Argon2 ハッシュ文字列を返す
  #[inline]
  pub fn as_hash(&self) -> &str {
    &self.hash
  }

  /// 平文パスワードがハッシュと一致するか検証
  pub fn verify<S: AsRef<str>>(&self, plain: S) -> bool {
    verify_hashed(plain.as_ref(), &self.hash).is_ok()
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  fn bd() -> NaiveDate {
    NaiveDate::from_ymd_opt(1990, 5, 15).unwrap()
  }

  #[test]
  fn new_hashes_and_zeroizes() {
    let pw = UserPassword::new(
      "A1b2C3d4!@#EfGhIjKlMnOpQrStUvWxYz$%&*()_+-=1234567890",
      true,
      "user",
      Some(bd()),
    )
    .unwrap()
    .unwrap();
    assert!(pw.as_hash().starts_with("$argon2id"));
    assert!(
      !pw
        .as_hash()
        .contains("A1b2C3d4!@#EfGhIjKlMnOpQrStUvWxYz$%&*()_+-=1234567890")
    );
  }

  #[test]
  fn verify_success() {
    let pw = UserPassword::new(
      "A1b2C3d4!@#EfGhIjKlMnOpQrStUvWxYz$%&*()_+-=1234567890",
      true,
      "user",
      Some(bd()),
    )
    .unwrap()
    .unwrap();
    assert!(pw.verify("A1b2C3d4!@#EfGhIjKlMnOpQrStUvWxYz$%&*()_+-=1234567890"));
  }

  #[test]
  fn verify_failure() {
    let pw = UserPassword::new(
      "A1b2C3d4!@#EfGhIjKlMnOpQrStUvWxYz$%&*()_+-=1234567890",
      true,
      "user",
      Some(bd()),
    )
    .unwrap()
    .unwrap();
    assert!(!pw.verify("WrongPass"));
  }
}
