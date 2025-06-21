use crate::{
  interfaces::http::error::{AppError, AppResult},
  utils::workspace,
};
use config::{Config, Environment, File};
use dotenvy::dotenv;
use serde::Deserialize;
use tracing as log;
use tracing_subscriber::filter::LevelFilter;
use urlencoding::encode;

/// アプリケーションのConfigの集約構造体
#[derive(Debug, Deserialize)]
pub struct AppConfig {
  pub app: App,
  pub log: Log,
  pub postgres: Postgres,
}

/// [app] section
#[derive(Debug, Deserialize)]
pub struct App {
  pub host: String,
  pub port: u16,
  pub version: String,
}

/// [log] section
#[derive(Debug, Deserialize)]
pub struct Log {
  pub level: String,
  pub format: String,
}

/// [postgres] section
#[derive(Debug, Deserialize)]
pub struct Postgres {
  pub host: String,
  pub port: u16,
  pub name: String,
  pub user: String,
  pub password: String,
  pub max_connections: u32,
}

impl AppConfig {
  /// Configを組立てて返す
  pub fn new() -> AppResult<Self> {
    // .envファイルの読み込み
    // 上記処理に失敗した場合は，警告を出力する
    if dotenv().is_err() {
      log::warn!(".env file not found or failed to load");
    }

    // `config/`のパス
    let config_dir = workspace::path("config", true)?;
    log::info!("Loading configuration from {:?}", config_dir);

    // `defaults.toml` → `development.toml` → `.env`の順で読み込む
    let builder = Config::builder()
      .add_source(File::from(config_dir.join("defaults.toml")).required(true))
      .add_source(File::from(config_dir.join("development.toml")).required(false))
      .add_source(Environment::with_prefix("APP").separator("__"))
      .add_source(Environment::with_prefix("POSTGRES").separator("__"))
      .add_source(Environment::with_prefix("LOG").separator("__"));

    builder
      .build()
      .map_err(|e| {
        AppError::InternalServerError(Some(format!(
          "Failed to build configuration from files ({:?}): {}",
          config_dir, e
        )))
      })?
      .try_deserialize()
      .map_err(|e| {
        AppError::InternalServerError(Some(format!(
          "Failed to deserialize configuration into AppConfig struct: {}",
          e
        )))
      })
  }

  /// postgres接続用URLを組立てて返す
  pub fn postgres_url(&self) -> String {
    format!(
      "postgres://{}:{}@{}:{}/{}",
      encode(&self.postgres.user),
      encode(&self.postgres.password),
      self.postgres.host,
      self.postgres.port,
      self.postgres.name
    )
  }
}

impl Log {
  /// LevelをtracingのLevelに変換して返す。
  pub fn level_filter(&self) -> LevelFilter {
    match self.level.to_lowercase().as_str() {
      "error" => LevelFilter::ERROR,
      "warn" => LevelFilter::WARN,
      "info" => LevelFilter::INFO,
      "debug" => LevelFilter::DEBUG,
      "trace" => LevelFilter::TRACE,
      // 設定値が適切でない場合は，infoを返す。
      other => {
        log::warn!("Unknown log level '{}', defaulting to INFO", other);
        LevelFilter::INFO
      }
    }
  }

  /// ログのフォーマットがJSONか，それ以外(PRETTY)か判定する。
  /// JSONの場合は，Trueを返す。
  pub fn is_json(&self) -> bool {
    matches!(self.format.to_lowercase().as_str(), "json" | "structured")
  }
}

#[cfg(test)]
mod tests {
  use super::AppConfig;
  /// AppConfig が正常に読み込めるか確認し，内容を表示する
  #[test]
  fn print_app_config() {
    let cfg = AppConfig::new().expect("Failed to load AppConfig");
    println!("{:#?}", cfg);
  }
}
