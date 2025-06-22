//! HTTP ハンドラ ― ユーザー関連

use crate::{
  application::user::{
    dto::{RegisterRequest, RegisterResponse},
    service::UserService,
  },
  domain::repository::{UserAuthRepository, UserRepository},
  interfaces::http::error::AppResult,
};
use async_trait::async_trait;
use axum::{Json, extract::Extension};

// ユーザー登録ハンドラ
pub async fn register_handler(
  Extension(service): Extension<UserService>,
  Json(request): Json<RegisterRequest>,
) -> AppResult<Json<RegisterResponse>> {
  let response = service.register(request).await?;
  Ok(Json(response))
}

// /// ユーザー登録ユースケースの振る舞いを抽象化する
// #[async_trait]
// pub trait UserRegisterUsecase: Send + Sync {
//   async fn register(&self, req: RegisterRequest) -> AppResult<RegisterResponse>;
// }

// /// UserServiceへのブランケットの実装
// #[async_trait]
// impl<R, A> UserRegisterUsecase for UserService<R, A>
// where
//   R: UserRepository + Send + Sync,
//   A: UserAuthRepository + Send + Sync,
// {
//   async fn register(&self, req: RegisterRequest) -> AppResult<RegisterResponse> {
//     UserService::register(self, req).await
//   }
// }

// /* Axum ハンドラ */
// /// POST /register
// pub async fn register_handler<Svc>(
//   Extension(svc): Extension<Svc>,
//   Json(req): Json<RegisterRequest>,
// ) -> AppResult<Json<RegisterResponse>>
// where
//   Svc: UserRegisterUsecase + Clone + Send + Sync + 'static,
// {
//   let res = svc.register(req).await?;
//   Ok(Json(res))
// }
