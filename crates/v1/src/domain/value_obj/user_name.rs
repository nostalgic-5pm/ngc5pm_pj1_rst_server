use crate::{
  domain::value_obj::normalized_string::NormalizedString,
  interfaces::http::error::{AppError, AppResult},
  utils::regex,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserName(pub NormalizedString);

impl UserName {
  const TARGET: &str = "ユーザー名(user_name)";
  const MIN_LEN: usize = 3;
  const MAX_LEN: usize = 64;

  pub fn new<S: AsRef<str>>(input: S, required: bool) -> AppResult<Option<Self>> {
    // 正規化・必須長さチェック
    let user_name_opt = NormalizedString::new(
      input,
      required,
      Self::TARGET,
      Some(Self::MIN_LEN),
      Some(Self::MAX_LEN),
    )?;

    // 空文字の場合はNoneを返す。
    let user_name = match user_name_opt {
      None => return Ok(None),
      Some(n) => n,
    };

    // 正規表現によるチェック
    if !regex::USER_NAME_REGEX.is_match(user_name.as_str()) {
      return Err(AppError::UnprocessableContent(Some(format!(
        "{}は以下のルールに従う必要があります。\n・使用可能文字：英数字，アンダースコア，ドット，ハイフン，プラス\n・先頭末尾は，英数字，アンダーバーのみ。\n・ドットは連続できない。",
        Self::TARGET
      ))));
    }

    // 正常時はUserName型のオブジェクトを返す。
    Ok(Some(Self(user_name)))
  }

  /// UserNameの実態への参照を返す。
  pub fn as_str(&self) -> &str {
    self.0.as_str()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn valid_usernames() -> Vec<&'static str> {
    vec![
      "user123",
      "user_name",
      "user.name",
      "user-name",
      "user+name",
      "user_name123",
      "User_123",
      "user__name",
      "a_b",
      "abc",
      "A1_",
      "user.name-123",
      "user+name-123",
      "user_name.name",
      "user_name-name",
      "user_name+name",
      "user123456789012345678901234567890123456789012345678901234567890", // 64 chars
    ]
  }

  fn invalid_usernames() -> Vec<&'static str> {
    vec![
      "",
      "ab",                                                                    // too short
      "a",                                                                     // too short
      "user..name",                                                            // consecutive dots
      ".username",                                                             // starts with dot
      "username.",                                                             // ends with dot
      "-username",                                                             // starts with hyphen
      "username-",                                                             // ends with hyphen
      "+username",                                                             // starts with plus
      "username+",                                                             // ends with plus
      "user name",                                                             // space
      "user@name",                                                             // invalid char
      "user!name",                                                             // invalid char
      "user#name",                                                             // invalid char
      "user$",                                                                 // invalid char
      "user%",                                                                 // invalid char
      "user^",                                                                 // invalid char
      "user&",                                                                 // invalid char
      "user*name",                                                             // invalid char
      "user/name",                                                             // invalid char
      "user\\name",                                                            // invalid char
      "user|name",                                                             // invalid char
      "user,name",                                                             // invalid char
      "user;name",                                                             // invalid char
      "user:name",                                                             // invalid char
      "user<name",                                                             // invalid char
      "user>name",                                                             // invalid char
      "user=name",                                                             // invalid char
      "user~name",                                                             // invalid char
      "user`name",                                                             // invalid char
      "user?name",                                                             // invalid char
      "user\tname",                                                            // tab
      "user\nname",                                                            // newline
      "user\rname",                                                            // carriage return
      "user12345678901234567890123456789012345678901234567890123456789012345", // 69 chars
    ]
  }

  #[test]
  fn test_valid_usernames() {
    for name in valid_usernames() {
      let result = UserName::new(name, true);
      assert!(result.is_ok(), "Should accept valid username: {}", name);
      let opt = result.unwrap();
      assert!(
        opt.is_some(),
        "Should return Some for valid username: {}",
        name
      );
      assert_eq!(opt.unwrap().as_str(), name);
    }
  }

  #[test]
  fn test_invalid_usernames() {
    for name in invalid_usernames() {
      let result = UserName::new(name, true);
      assert!(
        result.is_err() || result.as_ref().unwrap().is_none(),
        "Should reject invalid username: {}",
        name
      );
    }
  }

  #[test]
  fn test_optional_username_none() {
    // Not required, empty input should return Ok(None)
    let result = UserName::new("", false);
    assert!(result.is_ok());
    assert!(result.unwrap().is_none());
  }

  #[test]
  fn test_required_username_empty() {
    // Required, empty input should return error
    let result = UserName::new("", true);
    assert!(result.is_err());
  }

  #[test]
  fn test_min_length() {
    let min = "abc";
    let result = UserName::new(min, true);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().unwrap().as_str(), min);
  }

  #[test]
  fn test_max_length() {
    let max = "a".repeat(UserName::MAX_LEN);
    let result = UserName::new(&max, true);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().unwrap().as_str(), max);
  }

  #[test]
  fn test_over_max_length() {
    let over = "a".repeat(UserName::MAX_LEN + 1);
    let result = UserName::new(&over, true);
    assert!(result.is_err());
  }
}
