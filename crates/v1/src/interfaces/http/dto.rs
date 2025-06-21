/// APIレスポンスの標準フォーマットを定義する。
use serde::Serialize;

/// 正常時のレスポンス構造体。
#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
  /// 実際のレスポンスデータ。
  pub data: T,
  /// 結果の説明や追加情報を示すメッセージ。
  pub message: String,
  /// レスポンスが生成された時刻（UNIXタイムスタンプ）。
  pub timestamp: i64,
}

/// エラーレスポンス構造体。
#[derive(Debug, Serialize)]
pub struct ApiError {
  /// エラーに対応するHTTPステータスコード。
  pub status: u16,
  /// エラーの簡潔な要約。
  pub message: String,
  /// エラーの詳細な説明（オプション）。
  #[serde(skip_serializing_if = "Option::is_none")]
  pub detail: Option<String>,
  /// エラーが発生したインスタンスのURIや識別子（オプション）。
  #[serde(skip_serializing_if = "Option::is_none")]
  pub instance: Option<String>,
  /// エラーレスポンスが生成された時刻（UNIXタイムスタンプ）。
  pub timestamp: i64,
}
