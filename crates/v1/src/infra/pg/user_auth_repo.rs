//! PostgreSQL | user_auths テーブル Repository
//! --------------------------------------------------------------
//! ・INSERT を共通メソッド `insert_inner` に集約
//! ・Tx あり / なしをラップして呼び出せるようにする
//! --------------------------------------------------------------

use crate::{
  domain::{
    entity::user_auth::UserAuth,
    repository::UserAuthRepository,
    value_obj::{user_id::UserId, user_password::UserPassword},
  },
  interfaces::http::error::{AppError, AppResult},
};
use async_trait::async_trait;
use chrono::Utc;
use sqlx::{PgPool, Postgres, Transaction};

/// Tx 型エイリアス
pub type PgTx<'a> = Transaction<'a, Postgres>;

#[derive(Clone)]
pub struct PgUserAuthRepository {
  pool: PgPool,
}
impl PgUserAuthRepository {
  pub fn new(pool: PgPool) -> Self {
    Self { pool }
  }

  /* ===== INSERT (Tx なし) ===== */
  pub async fn insert(&self, a: &UserAuth) -> AppResult<()> {
    let mut tx = self.pool.begin().await.map_err(AppError::from)?;
    self.insert_inner(&mut tx, a).await?;
    tx.commit().await.map_err(AppError::from)
  }

  /* ===== INSERT (Tx あり) ===== */
  pub async fn insert_tx<'a>(&self, tx: &mut PgTx<'a>, a: &UserAuth) -> AppResult<()> {
    self.insert_inner(tx, a).await
  }

  /* ----------------------------------------------------------
   *  低レベル INSERT 本体
   * --------------------------------------------------------*/
  async fn insert_inner<'a>(&self, tx: &mut PgTx<'a>, a: &UserAuth) -> AppResult<()> {
    sqlx::query!(
      r#"
            INSERT INTO user_auths
              (user_id, current_hashed_password,
               prev_hashed_password_1, prev_hashed_password_2,
               login_fail_times, created_at, updated_at)
            VALUES ($1,$2,$3,$4,$5,$6,$7)
            "#,
      a.user_id.as_i64(),
      a.current_hash.as_hash(),
      a.prev_hash1.as_ref().map(|h| h.as_hash()),
      a.prev_hash2.as_ref().map(|h| h.as_hash()),
      a.login_fail_times as i16,
      a.created_at,
      a.updated_at,
    )
    .execute(&mut **tx)
    .await
    .map_err(AppError::from)?;
    Ok(())
  }

  /// ユーザーIDを指定して認証情報を取得するSQLを実行
  async fn do_find(&self, user_id: UserId) -> AppResult<Option<UserAuth>> {
    let row = sqlx::query_as!(
      AuthRow,
      r#"SELECT * FROM user_auths WHERE user_id=$1"#,
      user_id.as_i64()
    )
    .fetch_optional(&self.pool)
    .await
    .map_err(AppError::from)?;

    row.map(TryInto::<UserAuth>::try_into).transpose()
  }

  /// ユーザー認証情報を更新するSQLを実行
  async fn do_update(&self, a: &UserAuth) -> AppResult<()> {
    sqlx::query!(
      r#"UPDATE user_auths
        SET current_hashed_password = $1,
            prev_hashed_password_1  = $2,
            prev_hashed_password_2  = $3,
            login_fail_times        = $4,
            updated_at              = $5
      WHERE user_id = $6"#,
      a.current_hash.as_hash(),
      a.prev_hash1.as_ref().map(|h| h.as_hash()),
      a.prev_hash2.as_ref().map(|h| h.as_hash()),
      a.login_fail_times as i16,
      Utc::now(),
      a.user_id.as_i64()
    )
    .execute(&self.pool)
    .await
    .map_err(AppError::from)?;
    Ok(())
  }
}

/* UserAuthRepositoryの実装 */
#[async_trait]
impl UserAuthRepository for PgUserAuthRepository {
  async fn insert(&self, a: &UserAuth) -> AppResult<()> {
    self.insert(a).await
  }

  async fn find(&self, id: UserId) -> AppResult<Option<UserAuth>> {
    self.do_find(id).await
  }

  async fn update(&self, a: &UserAuth) -> AppResult<()> {
    self.do_update(a).await
  }
}

/* Row 構造体 & 変換 */
#[derive(sqlx::FromRow)]
struct AuthRow {
  user_id: i64,
  current_hashed_password: String,
  prev_hashed_password_1: Option<String>,
  prev_hashed_password_2: Option<String>,
  login_fail_times: i32,
  created_at: chrono::DateTime<Utc>,
  updated_at: chrono::DateTime<Utc>,
}

impl TryFrom<AuthRow> for UserAuth {
  type Error = AppError;
  fn try_from(r: AuthRow) -> Result<Self, Self::Error> {
    Ok(Self {
      user_id: UserId::new(r.user_id)?,
      current_hash: UserPassword::from_hash(r.current_hashed_password)?,
      prev_hash1: r
        .prev_hashed_password_1
        .map(UserPassword::from_hash)
        .transpose()?,
      prev_hash2: r
        .prev_hashed_password_2
        .map(UserPassword::from_hash)
        .transpose()?,
      login_fail_times: r.login_fail_times as u16,
      created_at: r.created_at,
      updated_at: r.updated_at,
    })
  }
}
