use unicode_general_category::{GeneralCategory::*, get_general_category};

/// 空白文字列をNoneに変換する。
pub fn blank_to_none(s: Option<String>) -> Option<String> {
  s.and_then(|string| {
    let str = string.trim();
    if str.is_empty() { None } else { Some(string) }
  })
}

/// 使用できないUnicodeカテゴリを列挙する。
pub fn is_forbidden_char(c: char) -> bool {
  match get_general_category(c) {
    // ZWJ / ZWNJ だけは例外的に許可
    Format if c == '\u{200C}' || c == '\u{200D}' => return false,

    // 以下のカテゴリはすべて禁止
    Control | Format | Surrogate | PrivateUse | Unassigned | LineSeparator | ParagraphSeparator => {
      return true;
    }
    _ => {}
  }
  // 個別の禁止コードポイント
  matches!(c as u32,
    // 双方向制御
    0x202A..=0x202E | 0x2066..=0x2069 | 0x200E | 0x200F |
    // タグ領域
    0xE0000..=0xE007F |
    // 非文字
    0xFDD0..=0xFDEF | 0xFFFE | 0xFFFF |
    0x1FFFE | 0x1FFFF | 0x2FFFE | 0x2FFFF | 0x3FFFE | 0x3FFFF |
    0x4FFFE | 0x4FFFF | 0x5FFFE | 0x5FFFF | 0x6FFFE | 0x6FFFF |
    0x7FFFE | 0x7FFFF | 0x8FFFE | 0x8FFFF | 0x9FFFE | 0x9FFFF |
    0xAFFFE | 0xAFFFF | 0xBFFFE | 0xBFFFF | 0xCFFFE | 0xCFFFF |
    0xDFFFE | 0xDFFFF | 0xEFFFE | 0xEFFFF | 0xFFFFE | 0xFFFFF |
    0x10FFFE | 0x10FFFF)
}
