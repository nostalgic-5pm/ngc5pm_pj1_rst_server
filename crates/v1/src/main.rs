//! エントリーポイント
//! --------------------------------------------------------------
//! ・Config 読み込み & Logger 初期化
//! ・PgPool 生成 → UserService へ注入
//! ・Axum ルータを構築して起動
//! --------------------------------------------------------------

use axum::{
  Router,
  extract::Extension,
  routing::{get, post},
};
use sqlx::postgres::PgPoolOptions;
use std::net::{IpAddr, SocketAddr};
use tokio::{net::TcpListener, signal};
use tracing as log;
use v1::{
  application::user::service::UserService,
  config::AppConfig,
  interfaces::http::{
    error::{AppError, AppResult},
    handler,
  },
  utils::logger::init_tracing,
};

#[tokio::main]
async fn main() -> AppResult<()> {
  // Configを読み込む
  let config = AppConfig::new()?;

  // ロギングの設定
  init_tracing(&config.log);
  log::info!("Configuration loaded: version {}", config.app.version);

  // Postgres接続
  // URL
  let postgres_url = config.postgres_url();
  // プール
  let postgres_pool = PgPoolOptions::new()
    .connect(&postgres_url)
    .await
    .map_err(|e| {
      AppError::InternalServerError(Some(format!("Failed to connect with postgres: {}", e)))
    })?;
  log::info!("Connected to the postgres");

  // リポジトリの初期化
  let svc = UserService::new(postgres_pool.clone());

  // ルーティング定義
  let app = Router::new()
    .route("/", get(root))
    .route("/register", post(handler::user::register_handler))
    .layer(Extension(svc))
    .layer(Extension(postgres_pool));

  // サーバーのアドレスを指定
  let ip: IpAddr = config
    .app
    .host
    .parse()
    .map_err(|e| AppError::InternalServerError(format!("Invalid IP address: {}", e).into()))?;
  let address = SocketAddr::new(ip, config.app.port);

  // 指定したアドレスでTCPリスナーをバインド
  let listener = TcpListener::bind(&address)
    .await
    .map_err(|e| AppError::InternalServerError(format!("Failed to bind: {}", e).into()))?;
  log::info!("▶ Server running on http://{}", &address);

  // Axumサーバーを起動
  axum::serve(listener, app.into_make_service())
    .with_graceful_shutdown(shutdown_signal())
    .await
    .map_err(|e| {
      AppError::InternalServerError(format!("Failed to start application: {}", e).into())
    })?;

  Ok(())
}

/// rootハンドラー
async fn root() -> String {
  "Hello, world!".to_string()
}

/// サーバーのシャットダウン
async fn shutdown_signal() {
  // Ctrl+C（SIGINT）シグナルを待機
  signal::ctrl_c()
    .await
    .expect("Failed to install Ctrl+C handler.");
  log::info!("Shutting down the server...");
}
