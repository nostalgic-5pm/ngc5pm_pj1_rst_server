//! 誕生日のVO

use crate::{
  domain::value_obj::normalized_string::NormalizedString,
  interfaces::http::error::{AppError, AppResult},
};
use chrono::{Datelike, Local, NaiveDate};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BirthDate(NaiveDate);

impl BirthDate {
  const TARGET: &str = "誕生日(birth_date)";
  const LEN: usize = 8;
  const MINIMUM_AGE: u32 = 18;

  /// String/&strからBirthDate型のオブジェクトを生成する。
  pub fn new<S: AsRef<str>>(input: S, required: bool) -> AppResult<Option<Self>> {
    // 文字列形式(YYYYmmdd)で誕生日を受け取り，正規化を行う。
    let birth_date_ns = NormalizedString::new(
      input,
      required,
      Self::TARGET,
      Some(Self::LEN),
      Some(Self::LEN),
    )?;

    // 空文字の場合はNoneを返す。
    let birth_date_ns = match birth_date_ns {
      None => return Ok(None),
      Some(ns) => ns,
    };

    // NaiveDateにパースできない場合はエラーを返す。
    let birth_date = match NaiveDate::parse_from_str(birth_date_ns.as_str(), "%Y%m%d") {
      Ok(bd) => bd,
      Err(_) => {
        return Err(AppError::UnprocessableContent(Some(format!(
          "{}は`YYYYmmdd`形式で入力してください。",
          Self::TARGET
        ))));
      }
    };

    // 入力値が未来日である場合はエラーを返す。
    let today = Self::today();
    if birth_date > today {
      return Err(AppError::UnprocessableContent(Some(format!(
        "{}は未来日を指定できません。",
        Self::TARGET
      ))));
    }
    Ok(Some(Self(birth_date)))
  }

  /// birth_dateの実態(NaiveDate)への参照を返す。
  pub fn as_naive_date(&self) -> &NaiveDate {
    &self.0
  }

  /// NaiveDateからBirthDate型のオブジェクトを生成する。
  pub fn from_naive_date(bd: NaiveDate) -> Self {
    BirthDate(bd)
  }

  /// 年齢(満年齢)を返す。
  pub fn calculate_to_age(&self) -> AppResult<u32> {
    let today = Self::today();
    let birthday = self.as_naive_date();
    let mut age = today.year() - birthday.year();

    // 今年の誕生日が来ていなければ1を引く。
    let birthday_this_year =
      NaiveDate::from_ymd_opt(today.year(), birthday.month(), birthday.day());
    let birthday_this_year = match birthday_this_year {
      Some(bd) => bd,
      None => {
        // 2/29生まれかつ今年が非閏年の場合は2/28を誕生日とみなす。
        if birthday.month() == 2 && birthday.day() == 29 {
          NaiveDate::from_ymd_opt(today.year(), 2, 28).unwrap()
        } else {
          // 入力された日付が不正のため，エラーを返す。。
          return Err(AppError::UnprocessableContent(Some(format!(
            "{}の値が不正です。",
            Self::TARGET
          ))));
        }
      }
    };
    if today < birthday_this_year {
      age -= 1;
    }

    // 年齢が負の値の場合はエラーを返す。
    if age < 0 {
      return Err(AppError::UnprocessableContent(Some(format!(
        "{}の値が不正です。",
        Self::TARGET
      ))));
    }
    Ok(age as u32)
  }

  /// 対象年齢かどうか
  pub fn is_of_age(&self) -> AppResult<bool> {
    let age = self.calculate_to_age()?;
    if age < Self::MINIMUM_AGE {
      return Err(AppError::UnprocessableContent(Some(
        "このコンテンツの対象年齢を満たしていません。".into(),
      )));
    }
    Ok(true)
  }
  /// 今日の日付を返す。
  fn today() -> NaiveDate {
    Local::now().date_naive()
  }
}
