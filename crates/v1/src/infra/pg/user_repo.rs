use crate::{
  domain::{
    entity::user::{User, UserRole, UserStatus},
    value_obj::{
      birth_date::BirthDate, email_address::EmailAddress, phone_number::PhoneNumber,
      public_id::PublicId, user_full_name::UserFullName, user_id::UserId, user_name::UserName,
    },
  },
  interfaces::http::error::{AppError, AppResult},
};
use chrono::Utc;
use sqlx::{PgPool, Postgres, Transaction};

/// `PgPool` を受け取り、ユーザー関連のリポジトリを初期化する
pub type PgTx<'a> = Transaction<'a, Postgres>;

/// ユーザーリポジトリ
#[derive(Clone)]
pub struct PgUserRepository {
  pool: PgPool,
}

// コンストラクタ
impl PgUserRepository {
  pub fn new(pool: PgPool) -> Self {
    Self { pool }
  }

  /// ユーザー登録
  /// ユーザー情報を受け取り、データベースに新規ユーザーを登録する
  pub async fn insert_ntx(&self, u: &User) -> AppResult<i64> {
    sqlx::query_scalar!(
      r#"
        INSERT INTO users
          (public_id, randomart, user_name,
            first_name, last_name,
            email, phone, birth_date,
            status, role,
            last_login_at, created_at, updated_at)
        VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13)
        RETURNING user_id
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
    .fetch_one(&self.pool)
    .await
    .map_err(AppError::from)
  }

  /// トランザクション内でのユーザー登録
  /// トランザクションを受け取り、ユーザー情報を登録する
  /// トランザクションは呼び出し元で管理される
  pub async fn insert_tx<'a>(&self, tx: &mut PgTx<'a>, u: &User) -> AppResult<i64> {
    sqlx::query_scalar!(
      r#"
        INSERT INTO users
          (public_id, randomart, user_name,
            first_name, last_name,
            email, phone, birth_date,
            status, role,
            last_login_at, created_at, updated_at)
        VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13)
        RETURNING user_id
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
    .fetch_one(&mut **tx) // PostgreSQL の RowStream を参照として渡す
    .await
    .map_err(AppError::from)
  }

  /// 主キー検索
  /// ユーザーIDを指定してStatus==Activeのユーザー情報を取得する
  /// ユーザーが存在しない場合は `None` を返す
  async fn find_by_user_id(&self, id: UserId) -> AppResult<Option<User>> {
    let row = sqlx::query_as!(
      UserRow,
      r#"SELECT
        user_id,
        public_id,
        randomart,
        user_name,
        first_name,
        last_name,
        email,
        phone,
        birth_date,
        status,
        role,
        last_login_at,
        created_at,
        updated_at
      FROM users
      WHERE user_id = $1 AND status = 0"#,
      id.as_i64(),
    )
    .fetch_optional(&self.pool)
    .await
    .map_err(AppError::from)?;

    row.map(TryInto::<User>::try_into).transpose()
  }

  /// user_name 検索
  /// ユーザー名を指定してStatus==Activeのユーザー情報を取得する
  /// ユーザーが存在しない場合は `None` を返す
  async fn find_by_username(&self, name: &UserName) -> AppResult<Option<User>> {
    let row = sqlx::query_as!(
      UserRow,
      r#"SELECT
        user_id, public_id, randomart, user_name,
        first_name, last_name, email, phone, birth_date,
        status, role, last_login_at, created_at, updated_at
      FROM users
      WHERE user_name = $1 AND status = 0"#,
      name.as_str()
    )
    .fetch_optional(&self.pool)
    .await
    .map_err(AppError::from)?;

    row.map(TryInto::<User>::try_into).transpose()
  }

  /// ユーザーのステータスを更新する
  async fn update_status(&self, u: &User) -> AppResult<()> {
    sqlx::query!(
      r#"UPDATE users
        SET status = $1,
          updated_at = $2
        WHERE user_id = $3"#,
      i16::from(u.status),
      Utc::now(),
      u.user_id.as_i64()
    )
    .execute(&self.pool)
    .await
    .map_err(AppError::from)?;
    Ok(())
  }
  /// ユーザーのロールを更新する
  async fn update_role(&self, u: &User) -> AppResult<()> {
    sqlx::query!(
      r#"UPDATE users
        SET role       = $1,
            updated_at = $2
        WHERE user_id  = $3"#,
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

/* 内部関数 */

/// users テーブルの行を表す構造体
#[derive(sqlx::FromRow)]
struct UserRow {
  user_id: i64,
  public_id: String,
  randomart: String,
  user_name: String,
  first_name: Option<String>,
  last_name: Option<String>,
  email: Option<String>,
  phone: Option<String>,
  birth_date: Option<chrono::NaiveDate>,
  status: i16,
  role: i16,
  last_login_at: Option<chrono::DateTime<Utc>>,
  created_at: chrono::DateTime<Utc>,
  updated_at: chrono::DateTime<Utc>,
}

/// `UserRow` から `User` への変換
impl TryFrom<UserRow> for User {
  type Error = AppError;
  fn try_from(r: UserRow) -> Result<Self, Self::Error> {
    Ok(Self {
      user_id: UserId::new(r.user_id)?,
      public_id: PublicId::from_string(&r.public_id, true)?.ok_or_else(|| {
        AppError::InternalServerError(format!("Invalid public_id in DB: {}", r.public_id).into())
      })?,
      randomart: r.randomart,
      user_name: UserName::new(&r.user_name, true)?.ok_or_else(|| {
        AppError::InternalServerError(format!("Invalid user_name in DB: {}", r.user_name).into())
      })?,
      full_name: match (r.first_name, r.last_name) {
        (Some(f), Some(l)) if !f.is_empty() || !l.is_empty() => UserFullName::new(f, l)?,
        _ => None,
      },
      email: r
        .email
        .and_then(|e| EmailAddress::new(e, true).transpose())
        .transpose()?,
      phone: r
        .phone
        .and_then(|p| PhoneNumber::new(p, true).transpose())
        .transpose()?,
      birth_date: r.birth_date.map(BirthDate::from_naive_date),
      status: UserStatus::from(r.status),
      role: UserRole::from(r.role),
      last_login_at: r.last_login_at,
      created_at: r.created_at,
      updated_at: r.updated_at,
    })
  }
}
