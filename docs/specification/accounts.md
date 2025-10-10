# 口座（Accounts）仕様書

## 1. 概要
本ドキュメントは、Ebisu APIにおける「口座（Account）」機能の仕様を記載します。口座はユーザーの金融口座や資産管理の単位を表します。

## 2. ドメインモデル
### AccountType（口座種別）
- id: 整数型（i64）
- name: 文字列型

### Account（口座）
- id: 文字列型（UUID）
- name: 文字列型
- account_type: AccountType
- memo: 文字列型（任意）
- created_at: 文字列型（ISO8601, 任意）
- updated_at: 文字列型（ISO8601, 任意）


## 3. API仕様

| APIエンドポイント                | HTTPメソッド | ボディ送信 | レスポンス         | 説明                                 |
|----------------------------------|--------------|------------|--------------------|--------------------------------------|
| /api/account                     | GET          | なし       | Account配列        | すべての口座情報を取得               |
| /api/account/type/{type_name}    | GET          | なし       | Account配列        | 指定した種別名の口座一覧を取得        |
| /api/account/{id}                | GET          | なし       | Account            | 指定IDの口座情報を取得               |
| /api/account                     | POST         | あり       | 作成件数・内容     | 新しい口座を作成                      |
| /api/account                     | PUT          | あり       | 更新件数           | 既存口座情報を更新                    |
| /api/account/{id}                | DELETE       | なし       | 削除件数           | 指定IDの口座を削除                    |

## 4. バリデーション・制約
- `id`はUUID形式で一意
- `name`は必須
- `account_type`は存在する種別であること
- `memo`は任意

## 5. エラーレスポンス例
- 404 Not Found: 該当口座なし
- 409 Conflict: ID重複
- 500 Internal Server Error: DB障害等

## 6. 備考
- 日付はISO8601文字列
- 口座種別は事前にマスタ登録されている必要あり

---
最終更新: 2025-10-10
