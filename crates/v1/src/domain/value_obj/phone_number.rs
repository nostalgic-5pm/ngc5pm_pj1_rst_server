use crate::{
  domain::value_obj::normalized_string::NormalizedString,
  interfaces::http::error::{AppError, AppResult},
  utils::regex,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhoneNumber(pub NormalizedString);

impl PhoneNumber {
  const TARGET: &str = "電話番号(phone_number)";
  const MIN_LEN: usize = 10;
  const MAX_LEN: usize = 11;

  pub fn new<S: AsRef<str>>(input: S, required: bool) -> AppResult<Option<Self>> {
    // 正規化・必須長さチェック
    let phone_number_opt = NormalizedString::new(
      input,
      required,
      Self::TARGET,
      Some(Self::MIN_LEN),
      Some(Self::MAX_LEN),
    )?;

    // 空文字の場合はNoneを返す。
    let phone_number = match phone_number_opt {
      None => return Ok(None),
      Some(n) => n,
    };

    // 正規表現によるチェック
    if !regex::PHONE_NUMBER_REGEX.is_match(phone_number.as_str()) {
      return Err(AppError::UnprocessableContent(Some(format!(
        "{}は以下のルールに従う必要があります。\n・使用可能文字：数字のみ\n・長さは{}文字以上{}文字以下\n・先頭は0で始める必要があります。",
        Self::TARGET,
        Self::MIN_LEN,
        Self::MAX_LEN,
      ))));
    }

    // 正常時はPhoneNumber型のオブジェクトを返す。
    Ok(Some(Self(phone_number)))
  }

  /// PhoneNumberを文字列への参照として返す。
  pub fn as_str(&self) -> &str {
    self.0.as_str()
  }
}
#[cfg(test)]
mod tests {
  use super::*;

  fn valid_phone_numbers() -> Vec<&'static str> {
    vec![
      "09012345678",
      "0801234567",
      "07012345678",
      "0123456789",
      "0312345678",
      //"０９０１２３４５６７８", // 正規化され通過する想定
    ]
  }

  fn invalid_phone_numbers() -> Vec<&'static str> {
    vec![
      "9012345678",    // does not start with 0
      "090-1234-5678", // contains hyphens
      "090123456",     // too short
      "090123456789",  // too long
      "abcdefghijk",   // non-numeric
      "",              // empty string
    ]
  }

  #[test]
  fn test_valid_phone_numbers() {
    for num in valid_phone_numbers() {
      let result = PhoneNumber::new(num, true);
      assert!(result.is_ok(), "Should accept valid phone number: {}", num);
      let phone = result.unwrap();
      assert!(phone.is_some());
      assert_eq!(phone.unwrap().as_str(), num);
    }
  }

  #[test]
  fn test_invalid_phone_numbers() {
    for num in invalid_phone_numbers() {
      let result = PhoneNumber::new(num, true);
      assert!(
        result.is_err() || result.as_ref().unwrap().is_none(),
        "Should reject invalid phone number: {}",
        num
      );
    }
  }

  #[test]
  fn test_optional_phone_number_none() {
    let result = PhoneNumber::new("", false);
    assert!(result.is_ok());
    assert!(result.unwrap().is_none());
  }

  #[test]
  fn test_required_empty_phone_number() {
    let result = PhoneNumber::new("", true);
    assert!(result.is_err());
  }

  #[test]
  fn test_phone_number_as_str() {
    let num = "09012345678";
    let phone = PhoneNumber::new(num, true).unwrap().unwrap();
    assert_eq!(phone.as_str(), num);
  }
}
