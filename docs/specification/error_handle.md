# エラーハンドリング仕様

## 概要

本プロジェクト（ebisu_api）では、API全体のエラーハンドリングを統一的に行うため、独自のエラー型 `AppError` とエラーレスポンス構造体 `ErrorResponse` を導入しています。
これにより、クライアントは一貫した形式でエラー情報を受け取ることができます。

---

## 1. エラー型の定義

### AppError（アプリケーションエラー）

- Rustのenum型で定義。
- 代表的なバリアント：
  - `BadRequest(String)`
  - `InvalidInput(String)`
  - `Unauthorized(String)`
  - `NotFound(String)`
  - `Conflict(String)`
  - `InternalServerError(String)`
- すべてのバリアントはエラーメッセージとして `String` 型を保持します。
- `thiserror`クレートを利用し、`std::error::Error`トレイトを実装しています。

### ErrorResponse（エラーレスポンス構造体）

- APIのエラーレスポンスは必ず以下のJSON形式で返却されます：

```json
{
  "error": {
    "code": <HTTPステータスコード>,
    "message": "エラーメッセージ"
  }
}
```

- Rust構造体：
  - `ErrorResponse { error: ErrorDetail }`
  - `ErrorDetail { code: u16, message: String }`

---

## 2. エラー発生時のレスポンス例

| HTTPステータス | バリアント               | 例メッセージ               | レスポンス例                                                            |
| -------------- | ------------------------ | -------------------------- | ----------------------------------------------------------------------- |
| 400            | BadRequest, InvalidInput | "リクエストが不正です"     | `{ "error": { "code": 400, "message": "Invalid input." } }`             |
| 401            | Unauthorized             | "認証が必要です"           | `{ "error": { "code": 401, "message": "Unauthorized." } }`              |
| 404            | NotFound                 | "リソースが見つかりません" | `{ "error": { "code": 404, "message": "Account not found." } }`         |
| 409            | Conflict                 | "IDが既に存在します"       | `{ "error": { "code": 409, "message": "Account ID already exists." } }` |
| 500            | InternalServerError      | "サーバ内部エラー"         | `{ "error": { "code": 500, "message": "Failed to fetch account." } }`   |

---

## 3. 実装上のポイント

- すべてのハンドラー関数は `Result<HttpResponse, AppError>` 型（エイリアス: HandlerResult）を返します。
- エラー発生時は `Err(AppError::Xxx(...))` を返却し、`ResponseError`トレイトの実装により自動的にJSON形式のエラーレスポンスが生成されます。
- エラーメッセージは `MessageHierarchy::get` などで多言語・定数管理が可能です。
- utoipaによるOpenAPIドキュメントにも `ErrorResponse` 型が反映されます。

---

## 4. クライアント実装者向け注意点

- すべてのAPIエラーは `error.code`（HTTPステータス）と `error.message`（詳細メッセージ）で判定してください。
- 仕様変更等でエラーメッセージが動的に変化する場合があります。`code`を主に判定し、`message`は参考情報として扱ってください。

---

## 5. 参考：関連ファイル

- `src/utils/error.rs` ... エラー型・レスポンス定義
- `src/handler/accounts.rs` ... ハンドラーでのエラー返却例
- OpenAPI: `/api/docs` でスキーマ確認可能
