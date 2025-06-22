//! UserService

use crate::{
  application::user::dto::{RegisterRequest, RegisterResponse},
  domain::{
    entity::user::{UserRole, UserStatus},
    entity::{user::User, user_auth::UserAuth},
    value_obj::{
      birth_date::BirthDate, email_address::EmailAddress, phone_number::PhoneNumber,
      public_id::PublicId, user_full_name::UserFullName, user_id::UserId, user_name::UserName,
      user_password::UserPassword,
    },
  },
  infra::pg::{user_auth_repo::PgUserAuthRepository, user_repo::PgUserRepository},
  interfaces::http::error::{AppError, AppResult},
  utils::randomart::generate_randomart,
};
use chrono::Utc;
use sqlx::PgPool;

/// `PgPool` を受け取り、ユーザー関連のリポジトリを初期化するサービス
#[derive(Clone)]
pub struct UserService {
  pool: PgPool,
  user_repo: PgUserRepository,
  auth_repo: PgUserAuthRepository,
}

impl UserService {
  /// コンストラクタ
  /// `PgPool` を受け取り、内部で `PgUserRepository` と `PgUserAuthRepository` を初期化する
  pub fn new(pool: PgPool) -> Self {
    Self {
      user_repo: PgUserRepository::new(pool.clone()),
      auth_repo: PgUserAuthRepository::new(pool.clone()),
      pool,
    }
  }

  /// ユーザー登録サービス
  /// ユーザー名とパスワードを受け取り、ユーザーと認証情報をデータベースに登録する
  pub async fn register(&self, request: RegisterRequest) -> AppResult<RegisterResponse> {
    // 内部関数[build_entities]を使用して，`VO`と`Entity`を構築する
    // リクエスト→ `VO` → `Entity`へと変換をする。`
    let (mut user, mut auth) = Self::build_entities(&request)?;

    // トランザクションを開始する
    let mut tx = self.pool.begin().await.map_err(AppError::from)?;

    // ユーザーを，users テーブルに INSERT する
    let new_id = self.user_repo.insert_tx(&mut tx, &user).await?;
    user.user_id = UserId::new(new_id)?; // 自動採番をセット

    // ユーザー認証情報を，user_auths テーブルに INSERT する
    auth.user_id = user.user_id;
    self.auth_repo.insert_tx(&mut tx, &auth).await?;

    // トランザクションをコミットする
    tx.commit().await.map_err(AppError::from)?;

    // 4. レスポンス DTO
    Ok(RegisterResponse {
      public_id: user.public_id.as_str().to_owned(),
      randomart: user.randomart,
    })
  }

  /* 内部関数  */

  /// Requestデータを受け取り、`User` と `UserAuth` のエンティティを生成する
  fn build_entities(req: &RegisterRequest) -> AppResult<(User, UserAuth)> {
    // ユーザー名とパスワードが空でないことをチェックする
    if req.user_name.trim().is_empty() || req.password.trim().is_empty() {
      return Err(AppError::UnprocessableContent(Some(
        "ユーザー名(user_name)及びパスワード(user_password)は必須です。".into(),
      )));
    }

    // 各種の`VO`を生成する
    let user_name = UserName::new(&req.user_name, true)?.unwrap();

    let password = UserPassword::new(&req.password, true, &req.user_name, req.birth_date)?.unwrap();

    let full_name = UserFullName::new(
      req.first_name.clone().unwrap_or_default(),
      req.last_name.clone().unwrap_or_default(),
    )?;

    // Optional 項目は `Option<_>` のまま VO 化
    let email = req
      .email
      .as_deref()
      .map(|e| EmailAddress::new(e, false))
      .transpose()?
      .flatten();

    let phone = req
      .phone
      .as_deref()
      .map(|p| PhoneNumber::new(p, false))
      .transpose()?
      .flatten();

    let birth_date = req.birth_date.map(BirthDate::from_naive_date);

    // Entityの生成
    let now = Utc::now();
    let public_id = PublicId::new();
    let randomart = generate_randomart(&public_id);

    // user_id は 0 でダミー。INSERT 後に上書きする
    let user = User {
      user_id: UserId::new(0)?,
      public_id: public_id.clone(),
      randomart: randomart.clone(),
      user_name,
      full_name,
      email,
      phone,
      birth_date,
      status: UserStatus::Pending,
      role: UserRole::User,
      last_login_at: None,
      created_at: now,
      updated_at: now,
    };

    let auth = UserAuth {
      user_id: user.user_id,
      current_hash: password,
      prev_hash1: None,
      prev_hash2: None,
      login_fail_times: 0,
      created_at: now,
      updated_at: now,
    };

    // ユーザーと認証情報のタプルを返す
    Ok((user, auth))
  }
}
