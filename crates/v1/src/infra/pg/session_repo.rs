//! Postgres 実装 ― sessions テーブル

use crate::{
  domain::{
    entity::session::Session,
    value_obj::{session_id::SessionId, user_id::UserId},
  },
  interfaces::http::error::{AppError, AppResult},
};
use sqlx::PgPool;

#[derive(Clone)]
pub struct PgSessionRepository {
  pool: PgPool,
}
impl PgSessionRepository {
  pub fn new(pool: PgPool) -> Self {
    Self { pool }
  }

  /* ---------- INSERT ---------- */
  pub async fn insert(&self, s: &Session) -> AppResult<()> {
    sqlx::query!(
      r#"
            INSERT INTO sessions
              (session_id, user_id, created_at, expires_at)
            VALUES ($1,$2,$3,$4)
            "#,
      s.session_id.as_uuid(),
      s.user_id.as_i64(),
      s.created_at,
      s.expires_at,
    )
    .execute(&self.pool)
    .await
    .map_err(AppError::from)?;
    Ok(())
  }

  /* ---------- SELECT ---------- */
  pub async fn find(&self, sid: SessionId) -> AppResult<Option<Session>> {
    let row = sqlx::query_as!(
      SessionRow,
      r#"SELECT * FROM sessions WHERE session_id=$1"#,
      sid.as_uuid()
    )
    .fetch_optional(&self.pool)
    .await
    .map_err(AppError::from)?;

    row.map(TryInto::<Session>::try_into).transpose()
  }

  /* ---------- DELETE ---------- */
  pub async fn delete(&self, sid: SessionId) -> AppResult<()> {
    sqlx::query!("DELETE FROM sessions WHERE session_id=$1", sid.as_uuid())
      .execute(&self.pool)
      .await
      .map_err(AppError::from)?;
    Ok(())
  }
}

/* -------- Row 構造体 & 変換 -------- */
#[derive(sqlx::FromRow)]
struct SessionRow {
  session_id: uuid::Uuid,
  user_id: i64,
  created_at: chrono::DateTime<chrono::Utc>,
  expires_at: chrono::DateTime<chrono::Utc>,
}

impl TryFrom<SessionRow> for Session {
  type Error = AppError;
  fn try_from(r: SessionRow) -> Result<Self, Self::Error> {
    Ok(Self {
      session_id: SessionId::from_string(r.session_id.to_string(), true)?.unwrap(),
      user_id: UserId::new(r.user_id)?,
      created_at: r.created_at,
      expires_at: r.expires_at,
    })
  }
}
