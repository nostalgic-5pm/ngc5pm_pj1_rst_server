//! HTTPレイヤ専用の上位Error型・Result型及び変換ロジック

use super::dto::ApiError;
use AppError::*;
use axum::{
  Json,
  http::StatusCode,
  response::{IntoResponse, Response},
};
use chrono::Utc;
use sqlx::Error as SqlxError;
use std::{borrow::Cow, string::String};
use thiserror::Error;
use tracing as log;

/// プロジェクト全体で使用するResult型
pub type AppResult<T> = Result<T, AppError>;

/// SQLSTATE（PostgreSQL）
mod sqlstate {
  pub const UNIQUE_VIOLATION: &str = "23505";
  pub const FK_VIOLATION: &str = "23503";
  pub const NOT_NULL_VIOLATION: &str = "23502";
  pub const CHECK_VIOLATION: &str = "23514";
}

/// HTTP レイヤの上位エラー
/// 各バリアントは，対応するHTTPステータスコードとOpt.のDetailを持つ。
#[derive(Debug, Error)]
pub enum AppError {
  #[error("Bad Request")]
  BadRequest(Option<String>),
  #[error("Unauthorized")]
  Unauthorized(Option<String>),
  #[error("Forbidden")]
  Forbidden(Option<String>),
  #[error("Not Found")]
  NotFound(Option<String>),
  #[error("Request Timeout")]
  RequestTimeout(Option<String>),
  #[error("Conflict")]
  Conflict(Option<String>),
  #[error("I'm a Teapot")]
  ImATeapot(Option<String>),
  #[error("Unprocessable Content")]
  UnprocessableContent(Option<String>),
  #[error("Internal Server Error")]
  InternalServerError(Option<String>),
}

impl AppError {
  /// HTTPステータス取得
  fn status_code(&self) -> StatusCode {
    match self {
      BadRequest(_) => StatusCode::BAD_REQUEST,
      Unauthorized(_) => StatusCode::UNAUTHORIZED,
      Forbidden(_) => StatusCode::FORBIDDEN,
      NotFound(_) => StatusCode::NOT_FOUND,
      RequestTimeout(_) => StatusCode::REQUEST_TIMEOUT,
      Conflict(_) => StatusCode::CONFLICT,
      ImATeapot(_) => StatusCode::IM_A_TEAPOT,
      UnprocessableContent(_) => StatusCode::UNPROCESSABLE_ENTITY,
      InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
  }

  /// コンストラクタで受け取ったDetail（無ければNone）を返す。
  fn detail(&self) -> Option<&String> {
    match self {
      BadRequest(d)
      | Unauthorized(d)
      | Forbidden(d)
      | NotFound(d)
      | RequestTimeout(d)
      | Conflict(d)
      | ImATeapot(d)
      | UnprocessableContent(d)
      | InternalServerError(d) => d.as_ref(),
    }
  }
}

impl IntoResponse for AppError {
  /// AppErrorをHTTPステータスコードに変換する。
  fn into_response(self) -> Response {
    let status = self.status_code();

    // ログを出力する。
    // (500系はError, それ以外はWarn)
    if status.is_server_error() {
      log::error!(error=?self, "Internal server error");
    } else {
      log::warn!(error=?self, "Client error");
    }

    // Statusに応じてResponseBodyを構築する。
    // （500系にはDetailを含めない。）
    let body = if status.is_server_error() {
      ApiError {
        status: status.as_u16(),
        message: status
          .canonical_reason()
          .unwrap_or("Internal server error")
          .to_string(),
        detail: None,
        instance: None,
        timestamp: Utc::now().timestamp(),
      }
    } else {
      ApiError {
        status: status.as_u16(),
        message: status.canonical_reason().unwrap_or("Error").to_string(),
        detail: self.detail().cloned(),
        instance: None,
        timestamp: Utc::now().timestamp(),
      }
    };

    (status, Json(body)).into_response()
  }
}

impl From<String> for AppError {
  /// String関係のエラーをAppErrorに変換する。
  fn from(s: String) -> Self {
    AppError::InternalServerError(Some(s))
  }
}

impl From<&str> for AppError {
  /// &str関係のエラーをAppErrorに変換する。
  fn from(s: &str) -> Self {
    AppError::InternalServerError(Some(s.to_owned()))
  }
}

impl From<SqlxError> for AppError {
  /// SqlxのエラーをAppErrorに変換する。
  fn from(err: SqlxError) -> Self {
    match err {
      SqlxError::RowNotFound => NotFound(Some("Resource not found".into())),
      SqlxError::PoolTimedOut => RequestTimeout(Some("Database timeout".into())),
      SqlxError::Database(ref db) => match db.code() {
        Some(Cow::Borrowed(sqlstate::UNIQUE_VIOLATION))
        | Some(Cow::Borrowed(sqlstate::FK_VIOLATION))
        | Some(Cow::Borrowed(sqlstate::NOT_NULL_VIOLATION))
        | Some(Cow::Borrowed(sqlstate::CHECK_VIOLATION)) => {
          Conflict(Some("Integrity violation".into()))
        }
        _code => InternalServerError(Some("Database internal error".into())),
      },
      // 型ごとに判定できる場合は，文字列化せずに判定する
      SqlxError::Io(ref io_err) if io_err.kind() == std::io::ErrorKind::TimedOut => {
        RequestTimeout(Some("Database timeout".into()))
      }
      SqlxError::PoolClosed => RequestTimeout(Some("Database pool closed".into())),
      e => {
        let msg = e.to_string();
        // msgに"timeout"が含まれていれば408エラー。
        if msg.contains("timeout") {
          RequestTimeout(Some("Database timeout".into()))
        } else {
          // その他不明なエラー
          InternalServerError(Some(format!("DB error: {msg}")))
        }
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  // status_code()で返すHTTPステータスコードが適切か。
  fn test_status_code_mapping() {
    assert_eq!(
      AppError::BadRequest(None).status_code(),
      StatusCode::BAD_REQUEST
    );
    assert_eq!(
      AppError::Unauthorized(None).status_code(),
      StatusCode::UNAUTHORIZED
    );
    assert_eq!(
      AppError::Forbidden(None).status_code(),
      StatusCode::FORBIDDEN
    );
    assert_eq!(
      AppError::NotFound(None).status_code(),
      StatusCode::NOT_FOUND
    );
    assert_eq!(
      AppError::RequestTimeout(None).status_code(),
      StatusCode::REQUEST_TIMEOUT
    );
    assert_eq!(AppError::Conflict(None).status_code(), StatusCode::CONFLICT);
    assert_eq!(
      AppError::ImATeapot(None).status_code(),
      StatusCode::IM_A_TEAPOT
    );
    assert_eq!(
      AppError::UnprocessableContent(None).status_code(),
      StatusCode::UNPROCESSABLE_ENTITY
    );
    assert_eq!(
      AppError::InternalServerError(None).status_code(),
      StatusCode::INTERNAL_SERVER_ERROR
    );
  }

  #[test]
  fn test_detail_extraction() {
    let detail = Some("detail".to_string());
    assert_eq!(
      AppError::BadRequest(detail.clone()).detail(),
      detail.as_ref()
    );
    assert_eq!(AppError::InternalServerError(None).detail(), None);
  }

  #[test]
  fn test_from_sqlx_row_not_found() {
    let err = SqlxError::RowNotFound;
    let app_err = AppError::from(err);
    match app_err {
      AppError::NotFound(Some(msg)) => assert_eq!(msg, "Resource not found"),
      _ => panic!("Expected NotFound variant"),
    }
  }

  #[test]
  fn test_from_sqlx_pool_timed_out() {
    let err = SqlxError::PoolTimedOut;
    let app_err = AppError::from(err);
    match app_err {
      AppError::RequestTimeout(Some(msg)) => assert_eq!(msg, "Database timeout"),
      _ => panic!("Expected RequestTimeout variant"),
    }
  }
}
