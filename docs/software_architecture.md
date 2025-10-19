# Ebisu API ソフトウェアアーキテクチャ概要

## 1. 概要
Ebisu APIはRust/Actix Webをベースとした個人向け家計簿アプリのWeb APIです。DI（依存性注入）設計、非同期処理、テスト容易性、拡張性を重視した構成となっています。

---

## 2. 技術スタック
- 言語: Rust (Edition 2024)
- Webフレームワーク: Actix Web
- DIコンテナ: Shaku
- DB: SQLite (SQLx)
- APIドキュメント: Utoipa
- CLI: Clap
- ロギング: env_logger, chrono

---

## 3. レイヤ構成

```
┌───────────────┐
│   main.rs     │  ...エントリポイント/CLI分岐
└─────┬─────────┘
      │
┌─────▼─────────┐
│  utils/       │  ...アプリ初期化, DI, ログ, CLI, exporter
└─────┬─────────┘
      │
┌─────▼─────────┐
│  handler/     │  ...APIハンドラ(Actix Web), DI, バリデーション
└─────┬─────────┘
      │
┌─────▼─────────┐
│  dao/         │  ...DBアクセス, SQLx, トランザクション
└─────┬─────────┘
      │
┌─────▼─────────┐
│  entity/      │  ...ドメインモデル, スキーマ, ToSchema
└───────────────┘
```

---

## 4. 主要コンポーネント

### 4.1. main.rs
- CLI引数パース（clap）
- サブコマンド分岐（export, version, サーバ起動）
- サーバ起動時はutils/app_setup経由でDI・DB・ルーティング初期化

### 4.2. utils/
- app_setup.rs: DIコンテナ（AppModule）、DBプール生成、サーバ構築
- logging.rs: ロガー初期化
- exporter.rs: OpenAPI仕様エクスポート
- options.rs, version.rs: CLI/バージョン管理

### 4.3. handler/
- accounts.rs: 口座APIハンドラ（CRUD, DI, バリデーション, エラーハンドリング）
- swagger_ui.rs: OpenAPI/Swagger UI
- DI設計（AccountHandler, AccountHandlerImpl）

### 4.4. dao/
- accounts.rs: 口座DAO（AccountDao, AccountDaoImpl）
- SQLxによるDBアクセス

### 4.5. entity/
- accounts.rs: Account, AccountType構造体（FromRow, ToSchema, Serialize/Deserialize）
- 他: credit_card, entries等

---

## 5. DI設計
- ShakuによるDIモジュール（AppModule）
    - AccountDao, AccountHandlerをDI
    - テスト用DIモジュール（TestAppModule）も分離可能
- DIによりハンドラ・DAOの疎結合化、テスト容易性向上

---

## 6. テスト設計
- unit_test/fixtures: モック・テストDB・パラメータ化DI
- handler/daoの単体・結合テスト（インメモリDB、パラメータ化モック）
- cargo testによる自動化

---

## 7. 拡張性・保守性
- DI設計により新規機能追加・テスト容易
- ドメインモデル・API仕様はdocs/specification, docs/designで管理
- コード・ドキュメント分離、PRレビュー・CI/CD対応可能

---

## 8. 図解（簡易）

```
main.rs
  └─ utils/app_setup.rs
        ├─ AppModule (DI)
        │    ├─ AccountDaoImpl
        │    └─ AccountHandlerImpl
        ├─ DBプール
        └─ ルーティング
  └─ handler/accounts.rs
        └─ dao/accounts.rs
            └─ entity/accounts.rs
```

---

## 9. 備考
- 詳細なクラス図は`docs/design/classes.md`参照
- API仕様は`docs/specification/accounts.md`等参照

---
最終更新: 2025-10-19
