use crate::domain::value_obj::{user_id::UserId, user_password::UserPassword};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct UserAuth {
  pub user_id: UserId,
  pub current_hash: UserPassword,
  pub prev_hash1: Option<UserPassword>,
  pub prev_hash2: Option<UserPassword>,
  pub login_fail_times: u16,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}
