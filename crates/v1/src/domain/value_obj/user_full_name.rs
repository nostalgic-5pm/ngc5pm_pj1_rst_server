use crate::{
  domain::value_obj::normalized_string::NormalizedString, interfaces::http::error::AppResult,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserFullName {
  pub first_name: NormalizedString,
  pub last_name: Option<NormalizedString>,
}

impl UserFullName {
  const FIRST_TARGET: &str = "名(FirstName)";
  const LAST_TARGET: &str = "姓(LastName)";
  const FIRST_REQUIRED: bool = false;
  const LAST_REQUIRED: bool = false;
  const MAX_LEN: usize = 64;

  pub fn new<S: AsRef<str>>(input_f: S, input_l: S) -> AppResult<Option<Self>> {
    // 正規化・必須長さチェック
    // first_name
    let f_opt = NormalizedString::new(
      input_f,
      Self::FIRST_REQUIRED,
      Self::FIRST_TARGET,
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
    if f_opt.is_none() && l_opt.is_none() {
      return Ok(None);
    }

    // first_nameが空でlast_nameに値がある場合はエラー
    if f_opt.is_none() && (l_opt.is_some()) {
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
      last_name: l_opt,
    }))
  }

  /// first_nameへの参照を返す
  pub fn first(&self) -> &str {
    self.first_name.as_str()
  }

  /// last_nameへの参照を返す
  pub fn last(&self) -> Option<&str> {
    self.last_name.as_ref().map(|s| s.as_str())
  }
}
