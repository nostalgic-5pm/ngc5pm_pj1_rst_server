//! 正規表現を定義する。

use once_cell::sync::Lazy;
use regex::Regex;

const ERROR_MESSAGE: &str = "正規表現のコンパイルに失敗しました。";

/// 電話番号正規表現
/// 日本国内の固定・携帯電話・IP電話などを想定する。
/// 入力は「ハイフン無し」，「先頭が0」，「全体で10 or 11桁」を想定する。
pub static PHONE_NUMBER_REGEX: Lazy<Regex> =
  Lazy::new(|| Regex::new(r"^0\d{9,10}$").expect(ERROR_MESSAGE));

/// eメールアドレス正規表現
/// ローカルパート：「英数字/_/+/-/.」，「ドットは連続しない」，「先頭末尾にドット禁止」。
/// ドメインラベル：「英数字/-」，「先頭末尾にハイフン禁止」，「末尾は必ずTLD」。
pub static EMAIL_ADDRESS_REGEX: Lazy<Regex> = Lazy::new(|| {
  Regex::new(
        r"(?i)^[A-Za-z0-9_+\-]+(?:\.[A-Za-z0-9_+\-]+)*@(?:[A-Za-z0-9](?:[A-Za-z0-9\-]*[A-Za-z0-9])?\.)+[A-Za-z]{2,}$"
    ).expect(ERROR_MESSAGE)
});

/// user_name正規表現
/// 英数字，アンダーバー，ドット，ハイフン，＋のみ。
/// 先頭末尾は，英数字，アンダーバーのみ。
/// ドットは連続しない。
pub static USER_NAME_REGEX: Lazy<Regex> = Lazy::new(|| {
  Regex::new(r"^(?:[A-Za-z0-9_]|[A-Za-z0-9_](?:[A-Za-z0-9_+\-]|\.[A-Za-z0-9_+\-])*[A-Za-z0-9_])$")
    .expect(ERROR_MESSAGE)
});

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_phone_number_regex_valid() {
    let valid_numbers = [
      "0123456789",
      "09012345678",
      "08012345678",
      "07012345678",
      "05012345678",
      "0312345678",
      "0000000000",
    ];
    for number in valid_numbers.iter() {
      assert!(
        PHONE_NUMBER_REGEX.is_match(number),
        "Should match: {}",
        number
      );
    }
  }

  #[test]
  fn test_phone_number_regex_invalid() {
    let invalid_numbers = [
      "1234567890",    // does not start with 0
      "090-1234-5678", // contains hyphens
      "090123456",     // too short
      "090123456789",  // too long
      "0abcdefghij",   // contains letters
    ];
    for number in invalid_numbers.iter() {
      assert!(
        !PHONE_NUMBER_REGEX.is_match(number),
        "Should not match: {}",
        number
      );
    }
  }

  #[test]
  fn test_email_address_regex_valid() {
    let valid_emails = [
      "user@example.com",
      "user.name+tag@example.co.jp",
      "user_name@example-domain.com",
      "user-name@sub.example.com",
      "u@e.co",
      "USER@EXAMPLE.COM",
      "a@b.qwertyuiopasdfghjkl",
    ];
    for email in valid_emails.iter() {
      assert!(
        EMAIL_ADDRESS_REGEX.is_match(email),
        "Should match: {}",
        email
      );
    }
  }

  #[test]
  fn test_email_address_regex_invalid() {
    let invalid_emails = [
      ".user@example.com",      // starts with dot
      "user.@example.com",      // ends with dot before @
      "user..name@example.com", // double dot in local part
      "user@-example.com",      // domain label starts with hyphen
      "user@example.com-",      // domain ends with hyphen
      "user@example",           // no TLD
      "user@.com",              // domain label starts with dot
      "user@com.",              // domain ends with dot
      "user@.com.",             // domain starts and ends with dot
    ];
    for email in invalid_emails.iter() {
      assert!(
        !EMAIL_ADDRESS_REGEX.is_match(email),
        "Should not match: {}",
        email
      );
    }
  }

  #[test]
  fn test_user_name_regex_valid() {
    let valid_usernames = [
      "u", // 長さ 1
      "_", // アンダースコアのみ
      "user",
      "user_name",
      "user.name",
      "user-name",
      "user+name",
      "user123",
      "A_USER",
    ];
    for username in valid_usernames.iter() {
      assert!(
        USER_NAME_REGEX.is_match(username),
        "Should match: {}",
        username
      );
    }
  }

  #[test]
  fn test_user_name_regex_invalid() {
    let invalid_usernames = [
      "",           // 空文字
      ".username",  // 先頭ドット
      "-username",  // 先頭ハイフン
      "+username",  // 先頭プラス
      "username.",  // 末尾ドット
      "username-",  // 末尾ハイフン
      "username+",  // 末尾プラス
      "user..name", // 連続ドット
      "user name",  // 空白含む
      "user@name",  // 記号 @
    ];
    for username in invalid_usernames.iter() {
      assert!(
        !USER_NAME_REGEX.is_match(username),
        "Should not match: {}",
        username
      );
    }
  }
}
