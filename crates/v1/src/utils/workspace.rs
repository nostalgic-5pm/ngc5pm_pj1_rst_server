//! utils/workspace.rs
//! ----------------------------------
//! workspace_root()  : `[workspace]`を含む`Cargo.toml`まで上方向探索
//! workspace_path()  : ルートからの相対パス & 必要なら存在確認
//! ----------------------------------

use crate::interfaces::http::error::{AppError, AppResult};
use qualified_do::{Resulted, qdo};
use std::{
  fs,
  path::{Path, PathBuf},
};

/// ワークスペースのルートディレクトリを返す
pub fn root() -> AppResult<PathBuf> {
  qdo! { Resulted {
    // 現在コンパイル中クレートのディレクトリ
    let mut dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    // クロージャでルート探索処理をラップし，Result化する
    root <- (|| {
      loop {

        // 現在のディレクトリにCargo.tomlがあるか確認する
        // Cargo.tomlが存在し，かつ[workspace]セクションがあればルートとみなす
        let cargo = dir.join("Cargo.toml");
        if cargo.is_file() && has_workspace_section(&cargo)? {
          return Ok::<_, AppError>(dir.clone());
        }

        // 親ディレクトリがなければ探索を終了する
        // （ルートが見つからなかった場合はエラーを返す）
        if !dir.pop() {
          return Err(AppError::InternalServerError(Some(
            "Failed to locate workspace root.".into(),
          )));
        }
      }
    })();

    // 探索結果を返す
    return root
  }}
}

/// ルート配下`relative`を返す
pub fn path<P: AsRef<Path>>(relative: P, must_exist: bool) -> AppResult<PathBuf> {
  qdo! { Resulted {
      root <- root();
      let path = root.join(&relative);

      // `must_exist == true`かつパスが見つからない場合は500エラー
      _ <- if must_exist && !path.exists() {
        Err(AppError::InternalServerError(Some(format!(
          "Expected {:?} directory at {:?}, but not found", relative.as_ref(), path
        ))))
      } else {
        Ok::<_, AppError>(())
      };

      return path
  }}
}

/// `Cargo.toml`内に`[workspace]`セクションが含まれるか判定する
fn has_workspace_section(cargo_toml: &Path) -> AppResult<bool> {
  // Cargo.tomlファイルの内容を文字列として読み込む
  let contents = fs::read_to_string(cargo_toml)
    .map_err(|e| AppError::InternalServerError(Some(e.to_string())))?;
  // [workspace]セクションが含まれているかどうかを判定
  Ok(contents.contains("[workspace]"))
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  // ワークスペースルートがCARGO_MANIFEST_DIRを含むか確認
  fn root_contains_manifest_dir() {
    let root = root().expect("should find root");
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    assert!(manifest.starts_with(&root));
  }

  #[test]
  // must_exist=falseの場合，存在しないパスでもエラーにならないことを確認
  fn nonexistent_path_allowed() {
    let p = path("this/does/not/exist", false).expect("path returned even if absent");
    assert!(!p.exists());
  }

  #[test]
  // must_exist=trueの場合，存在しないパスでエラーになることを確認
  fn nonexistent_path_denied() {
    let err = path("this/does/not/exist", true).expect_err("should error");
    assert!(matches!(err, AppError::InternalServerError(_)));
  }
}
