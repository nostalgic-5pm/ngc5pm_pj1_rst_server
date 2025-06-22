//! ユースケース層 – 入出力 DTO

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// ユーザー登録リクエスト (外部 I/F から受け取る)
#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RegisterRequest {
  pub user_name: String,
  pub password: String,
  pub first_name: Option<String>,
  pub last_name: Option<String>,
  pub email: Option<String>,
  pub phone: Option<String>,
  pub birth_date: Option<NaiveDate>,
}

/// ユーザー登録結果 (外部 I/F へ返す)
#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct RegisterResponse {
  pub public_id: String,
  pub randomart: String,
}
