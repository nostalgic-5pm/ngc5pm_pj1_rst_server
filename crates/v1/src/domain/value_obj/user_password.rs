use crate::{
  interfaces::http::error::{AppError, AppResult},
  utils::string::is_forbidden_char,
};
use chrono::NaiveDate;
use zxcvbn::{Score, zxcvbn};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserPassword(String);

impl UserPassword {
  const TARGET: &str = "パスワード(user_password)";
  const MIN_LEN: usize = 8;
  const MAX_LEN: usize = 64;
  const MIN_ZXCVBN_SCORE: Score = Score::Three;

  pub fn new<S: AsRef<str>>(
    input: S,
    required: bool,
    user_name: S,
    birth_date: Option<NaiveDate>,
  ) -> AppResult<Option<Self>> {
    // 正規化・必須長さチェック
    // 正規化：先頭末尾の空白をトリムするのみ
    let password = input.as_ref().trim();
    let lower_password = password.to_lowercase();

    if password.is_empty() && !required {
      return Ok(None);
    }
    if password.len() < Self::MIN_LEN || password.len() > Self::MAX_LEN {
      return Err(AppError::UnprocessableContent(Some(format!(
        "{}は{}文字以上、{}文字以下でなければなりません。",
        Self::TARGET,
        Self::MIN_LEN,
        Self::MAX_LEN
      ))));
    }

    // ユーザー名と誕生日がパスワードに含まれているかチェック
    let user_name = user_name.as_ref().to_lowercase();
    if lower_password.contains(&user_name.to_lowercase()) {
      return Err(AppError::UnprocessableContent(Some(format!(
        "{}にはユーザー名を含めることができません。",
        Self::TARGET
      ))));
    }
    if let Some(birth_date) = birth_date {
      let ymd = birth_date.format("%Y%m%d").to_string();
      let md = birth_date.format("%m%d").to_string();
      if lower_password.contains(&ymd) || lower_password.contains(&md) {
        return Err(AppError::UnprocessableContent(Some(format!(
          "{}には誕生日を含めることができません。",
          Self::TARGET
        ))));
      }
    }

    // 使用できないユニコード文字のチェック
    if lower_password.chars().any(is_forbidden_char) {
      return Err(AppError::UnprocessableContent(Some(format!(
        "{}には使用できない文字が含まれています。",
        Self::TARGET
      ))));
    }

    // パスワードの強度チェック
    let user_name_ref = user_name.as_str();
    let estimate = zxcvbn(&password, &[user_name_ref]);
    if estimate.score() < Self::MIN_ZXCVBN_SCORE {
      return Err(AppError::UnprocessableContent(Some(format!(
        "{}は強度が不十分です。より強力なパスワードを使用してください。",
        Self::TARGET
      ))));
    }

    // 正常時はUserPassword型のオブジェクトを返す
    Ok(Some(Self(password.to_string())))
  }

  /// UserPasswordの実態への参照を返す
  pub fn as_str(&self) -> &str {
    &self.0
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn valid_birth_date() -> NaiveDate {
    NaiveDate::from_ymd_opt(1990, 5, 15).unwrap()
  }

  #[test]
  fn test_valid_password() {
    let result = UserPassword::new(
      "StrongPassw0rd!",
      true,
      "username",
      Some(valid_birth_date()),
    );
    assert!(result.is_ok());
    assert!(result.unwrap().is_some());
  }

  #[test]
  fn test_password_too_short() {
    let result = UserPassword::new("short", true, "username", Some(valid_birth_date()));
    assert!(result.is_err());
  }

  #[test]
  fn test_password_too_long() {
    let long_pw = "a".repeat(UserPassword::MAX_LEN + 1);
    let result = UserPassword::new(
      long_pw,
      true,
      "username".to_string(),
      Some(valid_birth_date()),
    );
    assert!(result.is_err());
  }

  #[test]
  fn test_password_contains_username() {
    let result = UserPassword::new("username1234", true, "username", Some(valid_birth_date()));
    assert!(result.is_err());
  }

  #[test]
  fn test_password_contains_birthdate() {
    let result = UserPassword::new("mypassword19900515", true, "user", Some(valid_birth_date()));
    assert!(result.is_err());
  }

  #[test]
  fn test_password_contains_birthdate_md() {
    let result = UserPassword::new("mypassword0515", true, "user", Some(valid_birth_date()));
    assert!(result.is_err());
  }

  #[test]
  fn test_password_with_forbidden_char() {
    // Assuming is_forbidden_char returns true for '\u{200B}' (zero-width space)
    let result = UserPassword::new("validpass\u{200B}", true, "user", Some(valid_birth_date()));
    assert!(result.is_err());
  }

  #[test]
  fn test_password_strength_too_weak() {
    // "password" is likely too weak for zxcvbn
    let result = UserPassword::new("password", true, "user", Some(valid_birth_date()));
    assert!(result.is_err());
  }

  #[test]
  fn test_optional_password_not_required_and_empty() {
    let result = UserPassword::new("", false, "user", Some(valid_birth_date()));
    assert!(result.is_ok());
    assert!(result.unwrap().is_none());
  }

  #[test]
  fn test_trimmed_password() {
    let result = UserPassword::new(
      "   StrongPassw0rd!   ",
      true,
      "username",
      Some(valid_birth_date()),
    );
    assert!(result.is_ok());
    let pw = result.unwrap().unwrap();
    assert_eq!(pw.as_str(), "StrongPassw0rd!");
  }
}
