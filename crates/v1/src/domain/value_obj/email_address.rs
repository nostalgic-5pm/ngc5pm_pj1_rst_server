use crate::{
  domain::value_obj::normalized_string::NormalizedString,
  interfaces::http::error::{AppError, AppResult},
  utils::regex,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmailAddress(pub NormalizedString);

impl EmailAddress {
  const TARGET: &str = "メールアドレス(email_address)";
  const MIN_LEN: usize = 6;
  const MAX_LEN: usize = 254;

  pub fn new<S: AsRef<str>>(input: S, required: bool) -> AppResult<Option<Self>> {
    // 正規化・必須長さチェック
    let email_opt = NormalizedString::new(
      input,
      required,
      Self::TARGET,
      Some(Self::MIN_LEN),
      Some(Self::MAX_LEN),
    )?;

    // 空文字の場合はNoneを返す。
    let email = match email_opt {
      None => return Ok(None),
      Some(n) => n,
    };

    // 正規表現によるチェック
    if !regex::EMAIL_ADDRESS_REGEX.is_match(email.as_str()) {
      return Err(AppError::UnprocessableContent(Some(format!(
        "{}は有効なメールアドレス形式である必要があります。",
        Self::TARGET
      ))));
    }

    // 正常時はEmailAddress型のオブジェクトを返す。
    Ok(Some(Self(email)))
  }

  /// EmailAddressを文字列への参照として返す。
  pub fn as_str(&self) -> &str {
    self.0.as_str()
  }
}
#[cfg(test)]
mod tests {
  use super::*;

  fn valid_email() -> &'static str {
    "test.user@example.com"
  }

  fn invalid_email() -> &'static str {
    "invalid-email"
  }

  #[test]
  fn test_valid_email_address() {
    let result = EmailAddress::new(valid_email(), true);
    assert!(result.is_ok());
    let email = result.unwrap();
    assert!(email.is_some());
    assert_eq!(email.unwrap().as_str(), valid_email());
  }

  #[test]
  fn test_invalid_email_address_format() {
    let result = EmailAddress::new(invalid_email(), true);
    assert!(result.is_err());
  }

  #[test]
  fn test_empty_email_required() {
    let result = EmailAddress::new("", true);
    assert!(result.is_err());
  }

  #[test]
  fn test_empty_email_not_required() {
    let result = EmailAddress::new("", false);
    assert!(result.is_ok());
    assert!(result.unwrap().is_none());
  }

  #[test]
  fn test_email_too_short() {
    let short_email = "a@b.c";
    let result = EmailAddress::new(short_email, true);
    assert!(result.is_err());
  }

  #[test]
  fn test_email_too_long() {
    let long_local = "a".repeat(255 - 12);
    let long_email = format!("{}@example.com", long_local);
    let result = EmailAddress::new(long_email, true);
    assert!(result.is_err());
  }

  #[test]
  fn test_email_with_whitespace_is_normalized() {
    let email_with_spaces = "  test.user@example.com  ";
    let result = EmailAddress::new(email_with_spaces, true);
    assert!(result.is_ok());
    let email = result.unwrap().unwrap();
    assert_eq!(email.as_str(), "test.user@example.com");
  }
}
