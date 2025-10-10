use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// 口座種別を表す構造体。
#[derive(Debug, Serialize, Deserialize, FromRow, PartialEq, Eq, Clone)]
pub struct AccountType {
    pub id: i64,
    pub name: String,
}

/// 口座情報を表す構造体。accountsテーブルの1レコードに対応。
#[derive(Debug, Serialize, Deserialize, FromRow, PartialEq, Eq, Clone)]
pub struct Account {
    pub id: String,                 // 口座のユニークな識別子(UUID)
    pub name: String,               // 口座名
    pub account_type: AccountType,  // 口座種別
    pub memo: Option<String>,       // メモ
    pub created_at: Option<String>, // 作成日時
    pub updated_at: Option<String>, // 更新日時
}
