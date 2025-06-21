# 005 - エラー処理ライブラリにthiserrorを採用する

## Status

Accepted

## Context

アプリケーション開発においては，複数のエラー要因を一元的に扱うためのエラーハンドリング設計が必要となる。
Rustには，複数のエラーハンドリング系のライブラリが存在する。
代表的なものとして以下のようなものがある：

- anyhow: 一時的な開発・デバッグ，小規模な開発に有効
- thiserror: カスタムエラー定義に特化
- snafu: 文脈付きエラーを扱うための設計フレームワーク

## Considerations

以下の観点で比較を行う：

- 用途との適合性
- 外部エラーの統合容易性
- 学習コスト
- Axumとの親和性
- 採用実績

## Decision

thiserrorを採用する。
採用理由：

1. Rustにおけるドメインエラー設計に適している。
2. マクロを利用したシンプルな記法で，エラー構造・外部要因エラーからの変換処理等を実装できる。
3. `IntoResponse`がRestAPIとの親和性が高い。
4. 学習コストがsnafu程高くなさそう。

## Consequences

1. エラー構造の設計の見通しが良くなる。
2. 各層ごとにエラーを定義して，一元的にレスポンスに変換する構造を実現できる。

## Reference

[https://docs.rs/thiserror/latest/thiserror/]

## Notes

Owner: nostalgic.5pm
Proposed date: 2025-06-02
Last updated: 2025-06-02
