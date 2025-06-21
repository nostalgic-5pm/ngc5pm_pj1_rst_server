use crate::{
  domain::value_obj::normalized_string::NormalizedString, interfaces::http::error::AppResult,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserFullName {
  pub first_name: NormalizedString,
  pub middle_name: Option<NormalizedString>,
  pub last_name: Option<NormalizedString>,
}

impl UserFullName {
  const FIRST_TARGET: &str = "名(FirstName)";
  const MIDDLE_TARGET: &str = "中間名(MiddleName)";
  const LAST_TARGET: &str = "姓(LastName)";
  const FIRST_REQUIRED: bool = false;
  const MIDDLE_REQUIRED: bool = false;
  const LAST_REQUIRED: bool = false;
  const MAX_LEN: usize = 64;

  pub fn new<S: AsRef<str>>(input_f: S, input_m: S, input_l: S) -> AppResult<Option<Self>> {
    // 正規化・必須長さチェック
    // first_name
    let f_opt = NormalizedString::new(
      input_f,
      Self::FIRST_REQUIRED,
      Self::FIRST_TARGET,
      None,
      Some(Self::MAX_LEN),
    )?;

    // middle_name
    let m_opt = NormalizedString::new(
      input_m,
      Self::MIDDLE_REQUIRED,
      Self::MIDDLE_TARGET,
      None,
      Some(Self::MAX_LEN),
    )?;

    // last_name
    let l_opt = NormalizedString::new(
      input_l,
      Self::LAST_REQUIRED,
      Self::LAST_TARGET,
      None,
      Some(Self::MAX_LEN),
    )?;

    // すべて空ならNoneを返す
    if f_opt.is_none() && m_opt.is_none() && l_opt.is_none() {
      return Ok(None);
    }

    // first_nameが空でmiddle_nameまたはlast_nameに値がある場合はエラー
    if f_opt.is_none() && (m_opt.is_some() || l_opt.is_some()) {
      return Err(
        crate::interfaces::http::error::AppError::UnprocessableContent(Some(format!(
          "{}は必須のパラメータです。",
          Self::FIRST_TARGET
        ))),
      );
    }

    // first_nameがある場合はSomeで返す
    let first_name = match f_opt {
      Some(f) => f,
      None => unreachable!(), // 上記で全てのパターンを網羅している
    };

    Ok(Some(Self {
      first_name,
      middle_name: m_opt,
      last_name: l_opt,
    }))
  }

  /// first_nameへの参照を返す
  pub fn first(&self) -> &str {
    self.first_name.as_str()
  }

  /// middle_nameへの参照を返す
  pub fn middle(&self) -> Option<&str> {
    self.middle_name.as_ref().map(|s| s.as_str())
  }

  /// last_nameへの参照を返す
  pub fn last(&self) -> Option<&str> {
    self.last_name.as_ref().map(|s| s.as_str())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_user_full_name_with_all_fields() {
    let res = UserFullName::new("Taro", "Yamada", "Suzuki").unwrap();
    let name = res.unwrap();
    assert_eq!(name.first(), "Taro");
    assert_eq!(name.middle(), Some("Yamada"));
    assert_eq!(name.last(), Some("Suzuki"));
  }

  #[test]
  fn test_user_full_name_with_only_first_name() {
    let res = UserFullName::new("Taro", "", "").unwrap();
    let name = res.unwrap();
    assert_eq!(name.first(), "Taro");
    assert_eq!(name.middle(), None);
    assert_eq!(name.last(), None);
  }

  #[test]
  fn test_user_full_name_with_first_and_last_name() {
    let res = UserFullName::new("Taro", "", "Suzuki").unwrap();
    let name = res.unwrap();
    assert_eq!(name.first(), "Taro");
    assert_eq!(name.middle(), None);
    assert_eq!(name.last(), Some("Suzuki"));
  }

  #[test]
  fn test_user_full_name_with_first_and_middle_name() {
    let res = UserFullName::new("Taro", "Yamada", "").unwrap();
    let name = res.unwrap();
    assert_eq!(name.first(), "Taro");
    assert_eq!(name.middle(), Some("Yamada"));
    assert_eq!(name.last(), None);
  }

  #[test]
  fn test_user_full_name_missing_first_name_returns_none() {
    let res = UserFullName::new("", "Yamada", "Suzuki");
    assert!(res.is_err());
  }

  #[test]
  fn test_user_full_name_first_name_too_long() {
    let long_name = "a".repeat(UserFullName::MAX_LEN + 1);
    let res = UserFullName::new(long_name.as_str(), "", "").is_err();
    assert!(res);
  }

  #[test]
  fn test_user_full_name_middle_and_last_name_too_long() {
    let long_name = "a".repeat(UserFullName::MAX_LEN + 1);
    // middle_name too long
    let res = UserFullName::new("Taro", &long_name, "").is_err();
    assert!(res);
    // last_name too long
    let res = UserFullName::new("Taro", "", &long_name).is_err();
    assert!(res);
  }

  #[test]
  fn test_user_full_name_normalization() {
    // Assuming NormalizedString trims whitespace
    let res = UserFullName::new("  Taro  ", "  Yamada ", " Suzuki ").unwrap();
    let name = res.unwrap();
    assert_eq!(name.first(), "Taro");
    assert_eq!(name.middle(), Some("Yamada"));
    assert_eq!(name.last(), Some("Suzuki"));
  }
}
