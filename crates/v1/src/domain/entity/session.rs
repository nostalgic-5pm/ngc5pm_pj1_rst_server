use crate::domain::value_obj::{session_id::SessionId, user_id::UserId};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct Session {
  pub session_id: SessionId,
  pub user_id: UserId,
  pub created_at: DateTime<Utc>,
  pub expires_at: DateTime<Utc>,
}
