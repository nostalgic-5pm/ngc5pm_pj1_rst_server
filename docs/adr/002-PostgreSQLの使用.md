# 002 - RDBMSにPostgreSQLを採用する

## Status

Accepted

## Context

本プロジェクトは，アプリケーションのデータ永続化をRDBで実現する。

各RDBMSの性能とRustとの親和性を考慮して検討をする。

フリーで使用できるRDBMSであることを前提とする。

## Considerations

検討項目

- ライセンス
- Rustとの親和性
- 性能: {スケーラビリティ, 機能の豊富さなど}
- 運用実績

## Decision

RDBMSにPostgreSQLを採用する。

1. ライセンス：PostgreSQL LicenseによりアプリケーションがPostgreSQLに接続する場合のライセンス表示が不要。

2. Rustとの親和性：SQLx, SeaORM, Dieselなど各種ライブラリが整備されており，高い親和性を享受可能。

3. 性能：優秀な拡張性を有し，かつ高度なSQL機能を利用できる。

4. 運用実績：豊富な実装例がある。

## Consequences

1. PostgreSQLは使用経験があり，導入コストが低く迅速に実装に移ることができる。

2. SQLxを使用した接続例を発見済み。参考にすることでRust側の実装も迅速に実現できる想定。

## Notes

Owner: nostalgic.5pm
Proposed date: 2025-06-01
Last updated: 2025-06-01
