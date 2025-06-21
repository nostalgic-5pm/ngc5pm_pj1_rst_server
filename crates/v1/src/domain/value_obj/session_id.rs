use uuid::Uuid;

use crate::interfaces::http::error::{AppError, AppResult};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SessionId(Uuid);

impl SessionId {
  const TARGET: &str = "セッションID(session_id)";

  /// セッションIDを生成する
  pub fn new() -> Self {
    Self(Uuid::new_v4())
  }

  /// 文字列からUUIDを生成する
  pub fn from_string<S: AsRef<str>>(input: S, required: bool) -> AppResult<Option<Self>> {
    let input = input.as_ref().trim();
    if !required && input.is_empty() {
      return Ok(None);
    }
    match Uuid::parse_str(input) {
      Ok(u) => Ok(Some(Self(u))),
      Err(_) => Err(AppError::UnprocessableContent(Some(format!(
        "{}はUUIDの形式で入力してください。",
        Self::TARGET
      )))),
    }
  }

  /// セッションIDを文字列として返す。
  pub fn to_string(&self) -> String {
    self.0.to_string()
  }

  /// セッションIDの実態(Uuid)への参照を返す。
  pub fn as_uuid(&self) -> &Uuid {
    &self.0
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::collections::HashSet;

  #[test]
  fn test_new_generates_valid_uuid() {
    let session_id = SessionId::new();
    let uuid = session_id.as_uuid();
    assert_eq!(uuid.get_version_num(), 4);
  }

  #[test]
  fn test_from_string_valid_uuid_required_true() {
    let uuid = Uuid::new_v4();
    let input = uuid.to_string();
    let result = SessionId::from_string(&input, true).unwrap();
    assert_eq!(result.unwrap().as_uuid(), &uuid);
  }

  #[test]
  fn test_from_string_valid_uuid_required_false() {
    let uuid = Uuid::new_v4();
    let input = uuid.to_string();
    let result = SessionId::from_string(&input, false).unwrap();
    assert_eq!(result.unwrap().as_uuid(), &uuid);
  }

  #[test]
  fn test_from_string_empty_input_required_false() {
    let result = SessionId::from_string("", false).unwrap();
    assert!(result.is_none());
  }

  #[test]
  fn test_from_string_empty_input_required_true() {
    let result = SessionId::from_string("", true);
    assert!(result.is_err());
  }

  #[test]
  fn test_from_string_invalid_uuid() {
    let result = SessionId::from_string("not-a-uuid", true);
    assert!(result.is_err());
  }

  #[test]
  fn test_as_str_returns_uuid_string() {
    let session_id = SessionId::new();
    let uuid_str = session_id.to_string();
    assert_eq!(uuid_str, session_id.as_uuid().to_string());
  }

  #[test]
  fn test_equality_and_hash() {
    let uuid = Uuid::new_v4();
    let s1 = SessionId(uuid);
    let s2 = SessionId(uuid);
    assert_eq!(s1, s2);
    let mut set = HashSet::new();
    set.insert(s1.clone());
    assert!(set.contains(&s2));
  }
}
