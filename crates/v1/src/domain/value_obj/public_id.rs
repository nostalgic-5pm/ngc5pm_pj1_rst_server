use crate::interfaces::http::error::{AppError, AppResult};
use nid::Nanoid;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PublicId(Nanoid);

impl PublicId {
  const TARGET: &str = "公開ID(public_id)";
  const LEN: usize = 21;

  /// 公開IDを生成する
  pub fn new() -> Self {
    Self(Nanoid::new())
  }

  /// 文字列からPublicIdを生成する
  pub fn from_string<S: AsRef<str>>(input: S, required: bool) -> AppResult<Option<Self>> {
    let input = input.as_ref().trim();
    if !required && input.is_empty() {
      return Ok(None);
    }
    if input.len() != Self::LEN {
      return Err(AppError::UnprocessableContent(Some(format!(
        "{}は{}文字で入力してください。",
        Self::TARGET,
        Self::LEN
      ))));
    }

    match Nanoid::try_from_str(input) {
      Ok(nanoid) => Ok(Some(Self(nanoid))),
      Err(_) => Err(AppError::UnprocessableContent(Some(format!(
        "{}はNanoidの形式[A-Za-z0-9_-]で入力してください。",
        Self::TARGET,
      )))),
    }
  }

  /// 公開IDを文字列への参照として返す。
  pub fn as_str(&self) -> &str {
    self.0.as_str()
  }

  /// 公開IDの実態(Nanoid)への参照を返す。
  pub fn as_nanoid(&self) -> &Nanoid {
    &self.0
  }
}

impl Default for PublicId {
  fn default() -> Self {
    Self::new()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_new_generates_valid_public_id() {
    let public_id = PublicId::new();
    let id_str = public_id.as_str();
    assert_eq!(id_str.len(), PublicId::LEN);
    assert!(Nanoid::<{ PublicId::LEN }>::try_from_str(id_str).is_ok());
  }

  #[test]
  fn test_from_string_valid_input() {
    let public_id = PublicId::new();
    let id_str = public_id.as_str().to_string();
    let result = PublicId::from_string(&id_str, true);
    assert!(result.is_ok());
    let opt = result.unwrap();
    assert!(opt.is_some());
    assert_eq!(opt.unwrap().as_str(), id_str);
  }

  #[test]
  fn test_from_string_invalid_length() {
    let invalid = "short";
    let result = PublicId::from_string(invalid, true);
    assert!(matches!(
      result,
      Err(AppError::UnprocessableContent(Some(_)))
    ));
  }

  #[test]
  fn test_from_string_invalid_format() {
    let invalid = format!("{}!", "x".repeat(PublicId::LEN - 1));
    let result = PublicId::from_string(&invalid, true);
    assert!(matches!(
      result,
      Err(AppError::UnprocessableContent(Some(_)))
    ));
  }

  #[test]
  fn test_from_string_empty_optional() {
    let result = PublicId::from_string("", false);
    assert!(result.is_ok());
    assert!(result.unwrap().is_none());
  }

  #[test]
  fn test_from_string_empty_required() {
    let result = PublicId::from_string("", true);
    assert!(matches!(
      result,
      Err(AppError::UnprocessableContent(Some(_)))
    ));
  }

  #[test]
  fn test_as_nanoid_returns_inner() {
    let public_id = PublicId::new();
    let nanoid = public_id.as_nanoid();
    assert_eq!(nanoid.as_str(), public_id.as_str());
  }
}
