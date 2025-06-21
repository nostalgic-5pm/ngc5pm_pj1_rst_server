use crate::{
  domain::{
    entity::user::{User, UserRole, UserStatus},
    repository::UserRepository,
    value_obj::{
      birth_date::BirthDate, email_address::EmailAddress, phone_number::PhoneNumber,
      public_id::PublicId, user_full_name::UserFullName, user_id::UserId, user_name::UserName,
    },
  },
  interfaces::http::error::{AppError, AppResult},
};
use async_trait::async_trait;
use chrono::Utc;
use sqlx::{PgPool, Row};

pub struct PgUserRepository {
  pool: PgPool,
}
impl PgUserRepository {
  pub fn new(pool: PgPool) -> Self {
    Self { pool }
  }
}

#[async_trait]
impl UserRepository for PgUserRepository {
  async fn insert(&self, u: &User) -> AppResult<()> {
    sqlx::query!(
      r#"
        INSERT INTO users
          (public_id, randomart, user_name,
            first_name, last_name,
            email, phone, birth_date,
            status, role,
            last_login_at, created_at, updated_at)
        VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13)
        "#,
      u.public_id.as_str(),
      u.randomart,
      u.user_name.as_str(),
      u.full_name.as_ref().map(|n| n.first()),
      u.full_name.as_ref().and_then(|n| n.last()),
      u.email.as_ref().map(|e| e.as_str()),
      u.phone.as_ref().map(|p| p.as_str()),
      u.birth_date.as_ref().map(|b| b.as_naive_date()),
      i16::from(u.status),
      i16::from(u.role),
      u.last_login_at,
      u.created_at,
      u.updated_at,
    )
    .execute(&self.pool)
    .await
    .map_err(AppError::from)?;
    Ok(())
  }

  async fn find_by_id(&self, id: UserId) -> AppResult<Option<User>> {
    let rec = sqlx::query(
      "SELECT * FROM users WHERE user_id=$1 AND status<>4", // 4: Deleted
    )
    .bind(id.as_i64())
    .fetch_optional(&self.pool)
    .await
    .map_err(AppError::from)?;

    rec.map(row_to_user).transpose()
  }

  async fn find_by_username(&self, name: &UserName) -> AppResult<Option<User>> {
    let rec = sqlx::query("SELECT * FROM users WHERE user_name=$1")
      .bind(name.as_str())
      .fetch_optional(&self.pool)
      .await
      .map_err(AppError::from)?;
    rec.map(row_to_user).transpose()
  }

  async fn update(&self, u: &User) -> AppResult<()> {
    sqlx::query!(
      "UPDATE users SET status=$1, role=$2, updated_at=$3 WHERE user_id=$4",
      i16::from(u.status),
      i16::from(u.role),
      Utc::now(),
      u.user_id.as_i64()
    )
    .execute(&self.pool)
    .await
    .map_err(AppError::from)?;
    Ok(())
  }
}

/* ---------- private helpers ---------- */
fn row_to_user(r: sqlx::postgres::PgRow) -> AppResult<User> {
  Ok(User {
    user_id: UserId::new(r.get::<i64, _>("user_id"))?,
    public_id: PublicId::from_string(r.get::<&str, _>("public_id"), true)?.unwrap(),
    randomart: r.get("randomart"),
    user_name: UserName::new(r.get::<&str, _>("user_name"), true)?.unwrap(),
    full_name: {
      let first: Option<&str> = r.get("first_name");
      let last: Option<&str> = r.get("last_name");
      match (first, last) {
        (Some(f), Some(l)) if !f.is_empty() || !l.is_empty() => UserFullName::new(f, "", l)?,
        _ => None,
      }
    },
    email: r
      .get::<Option<&str>, _>("email")
      .and_then(|e| EmailAddress::new(e, true).transpose())
      .transpose()?,
    phone: r
      .get::<Option<&str>, _>("phone")
      .and_then(|p| PhoneNumber::new(p, true).transpose())
      .transpose()?,
    birth_date: r
      .get::<Option<chrono::NaiveDate>, _>("birth_date")
      .map(BirthDate::from_naive_date),
    status: UserStatus::from(r.get::<i16, _>("status")),
    role: UserRole::from(r.get::<i16, _>("role")),
    last_login_at: r.get("last_login_at"),
    created_at: r.get("created_at"),
    updated_at: r.get("updated_at"),
  })
}
