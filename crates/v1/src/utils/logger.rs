use crate::config::Log;
use tracing_subscriber::{
  fmt::{self, time::UtcTime},
  layer::SubscriberExt,
  util::SubscriberInitExt,
};

pub fn init_tracing(config: &Log) {
  // filter = Configで設定されているLogのレベル
  let filter = config.level_filter();

  // ログのフォーマットを定義する
  let fmt_layer = fmt::layer()
    .with_timer(UtcTime::rfc_3339())
    .with_level(true)
    .with_target(false)
    // .with_thread_ids(true)
    // .with_thread_names(true)
    ;

  // Json，またはPrettyでフォーマットをする
  if config.is_json() {
    tracing_subscriber::registry()
      .with(fmt_layer.json())
      .with(filter)
      .init();
  } else {
    tracing_subscriber::registry()
      .with(fmt_layer.pretty())
      .with(filter)
      .init();
  }
}
