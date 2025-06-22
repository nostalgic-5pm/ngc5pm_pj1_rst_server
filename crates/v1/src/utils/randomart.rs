//! PublicID(nanoid)をハッシュ化して，その値を使用して
//! Drunken Bishopアルゴリズムでランダムアートを生成する。

use crate::domain::value_obj::public_id::PublicId;
use sha3::{Digest, Sha3_384};

/// PublicIDからランダムアート文字列を生成する。
pub fn generate_randomart(public_id: &PublicId) -> String {
  let public_id_str = public_id.as_str();

  let fingerprint = {
    // state 再利用
    let mut hasher = Sha3_384::new();
    hasher.update(public_id_str.as_bytes());
    hasher.finalize()
  };

  // Drunken Bishopグリッドを生成
  let (grid, start, end) = _generate_drunken_bishop_grid(&fingerprint);

  // 上辺に表示する固定文字列
  let top_msg = "[your_id]";
  // 下辺に表示する固定文字列
  let bottom_msg = "[SHA3-384]";

  _render_drunken_bishop_art(&grid, start, end, top_msg, bottom_msg)
}

/// Drunken Bishop のグリッドを生成する
type DrunkenBishopGridResult = (Vec<Vec<u8>>, (usize, usize), (usize, usize));
fn _generate_drunken_bishop_grid(data: &[u8]) -> DrunkenBishopGridResult {
  let rows = 9;
  let cols = 23;
  let mut grid = vec![vec![0u8; cols]; rows];

  // スタート位置は中央
  let mut row = rows as i32 / 2;
  let mut col = cols as i32 / 2;
  let start_position = (row as usize, col as usize);

  // 各バイトを2ビットずつ取り出し、移動
  for &byte in data.iter() {
    let mut b = byte;
    for _ in 0..4 {
      let dx = if (b & 0x01) != 0 { 1 } else { -1 };
      let dy = if (b & 0x02) != 0 { 1 } else { -1 };

      row = (row + dy).clamp(0, rows as i32 - 1);
      col = (col + dx).clamp(0, cols as i32 - 1);
      grid[row as usize][col as usize] = grid[row as usize][col as usize].saturating_add(1);

      b >>= 2;
    }
  }

  let end_position = (row as usize, col as usize);
  (grid, start_position, end_position)
}

/// 幅 `cols` の内側中央に `msg` を右左パディングして枠線を作成
fn make_border_with_msg(cols: usize, msg: &str) -> String {
  if msg.len() >= cols {
    // メッセージがはみ出す場合はそのまま＋両端記号のみ
    format!("+{}+", msg)
  } else {
    let pad_left = (cols - msg.len()) / 2;
    let pad_right = cols - msg.len() - pad_left;
    format!("+{}{}{}+", "-".repeat(pad_left), msg, "-".repeat(pad_right))
  }
}

/// グリッドと開始／終了位置、上下メッセージをもとにアートをレンダリング
fn _render_drunken_bishop_art(
  grid: &[Vec<u8>],
  start_position: (usize, usize),
  end_position: (usize, usize),
  top_msg: &str,
  bottom_msg: &str,
) -> String {
  let rows = grid.len();
  let cols = if rows > 0 { grid[0].len() } else { 0 };
  let (sr, sc) = start_position;
  let (er, ec) = end_position;

  // カウント値 → シンボル変換表
  let symbols = [
    ' ', '.', 'o', '+', '=', '*', 'B', 'O', 'X', '@', '%', '&', '#', '/', '^',
  ];

  let mut lines = Vec::with_capacity(rows + 2);

  // 上辺
  lines.push(make_border_with_msg(cols, top_msg));

  // 本体行
  for (r, row_vec) in grid.iter().enumerate() {
    let mut line = String::with_capacity(cols + 2);
    line.push('|');

    for (c, &cnt) in row_vec.iter().enumerate() {
      if r == sr && c == sc {
        line.push('S');
      } else if r == er && c == ec {
        line.push('E');
      } else {
        let idx = usize::min(cnt as usize, symbols.len() - 1);
        line.push(symbols[idx]);
      }
    }

    line.push('|');
    lines.push(line);
  }

  // 下辺
  lines.push(make_border_with_msg(cols, bottom_msg));

  lines.join("\n")
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test_random_art_prints() {
    let public_id = PublicId::new();
    let art = generate_randomart(&public_id);
    println!("\n{}\n", art);
  }
}
