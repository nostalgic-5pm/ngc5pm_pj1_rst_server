use crate::{
  domain::{
    entity::{session::Session, user::User, user_auth::UserAuth},
    value_obj::{session_id::SessionId, user_id::UserId, user_name::UserName},
  },
  interfaces::http::error::AppResult,
};
use async_trait::async_trait;

#[async_trait]
pub trait UserRepository: Send + Sync {
  async fn insert(&self, u: &User) -> AppResult<()>;
  async fn find_by_id(&self, id: UserId) -> AppResult<Option<User>>;
  async fn find_by_username(&self, name: &UserName) -> AppResult<Option<User>>;
  async fn update(&self, u: &User) -> AppResult<()>;
}

#[async_trait]
pub trait UserAuthRepository: Send + Sync {
  async fn insert(&self, a: &UserAuth) -> AppResult<()>;
  async fn find(&self, id: UserId) -> AppResult<Option<UserAuth>>;
  async fn update(&self, a: &UserAuth) -> AppResult<()>;
}

#[async_trait]
pub trait SessionRepository: Send + Sync {
  async fn insert(&self, s: &Session) -> AppResult<()>;
  async fn find(&self, id: SessionId) -> AppResult<Option<Session>>;
  async fn delete(&self, id: SessionId) -> AppResult<()>;
}
