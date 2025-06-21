use crate::domain::value_obj::{
  birth_date::BirthDate, email_address::EmailAddress, phone_number::PhoneNumber,
  public_id::PublicId, user_full_name::UserFullName, user_id::UserId, user_name::UserName,
};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UserStatus {
  Active,
  Pending,
  Deactivated,
  Suspended,
  Deleted,
  Archived,
}
impl From<i16> for UserStatus {
  fn from(v: i16) -> Self {
    match v {
      0 => Self::Active,
      1 => Self::Pending,
      2 => Self::Deactivated,
      3 => Self::Suspended,
      4 => Self::Deleted,
      5 => Self::Archived,
      _ => panic!("不正なユーザーステータス値: {}", v),
    }
  }
}
impl From<UserStatus> for i16 {
  fn from(s: UserStatus) -> Self {
    match s {
      UserStatus::Active => 0,
      UserStatus::Pending => 1,
      UserStatus::Deactivated => 2,
      UserStatus::Suspended => 3,
      UserStatus::Deleted => 4,
      UserStatus::Archived => 5,
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UserRole {
  Guest,
  User,
  Support,
  Moderator,
  Admin,
  SuperAdmin,
}
impl From<i16> for UserRole {
  fn from(v: i16) -> Self {
    match v {
      0 => Self::User,
      1 => Self::Guest,
      2 => Self::Support,
      3 => Self::Moderator,
      4 => Self::Admin,
      5 => Self::SuperAdmin,
      _ => panic!("不正なユーザーロール値: {}", v),
    }
  }
}
impl From<UserRole> for i16 {
  fn from(r: UserRole) -> Self {
    match r {
      UserRole::User => 0,
      UserRole::Guest => 1,
      UserRole::Support => 2,
      UserRole::Moderator => 3,
      UserRole::Admin => 4,
      UserRole::SuperAdmin => 5,
    }
  }
}

#[derive(Debug, Clone)]
pub struct User {
  pub user_id: UserId,
  pub public_id: PublicId,
  pub randomart: String,
  pub user_name: UserName,
  pub full_name: Option<UserFullName>,
  pub email: Option<EmailAddress>,
  pub phone: Option<PhoneNumber>,
  pub birth_date: Option<BirthDate>,
  pub status: UserStatus,
  pub role: UserRole,
  pub last_login_at: Option<DateTime<Utc>>,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}
