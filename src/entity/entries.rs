use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// 出納タイプの構造体
#[derive(Debug, Serialize, Deserialize, FromRow, PartialEq, Eq, Clone)]
pub struct EntryType {
    pub id: i64,           // 出納タイプのユニークな識別子
    pub type_name: String, // 出納タイプ名（例: 収入、支出、振替）
}

/// 出納データの構造体
#[derive(Debug, Serialize, Deserialize, FromRow, PartialEq, Eq, Clone)]
pub struct Entry {
    pub entry_id: String,           // 出納のユニークな識別子
    pub entry_date: String,         // 出納日付
    pub entry_type: EntryType,      // 出納タイプ
    pub amount: i64,                // 金額
    pub memo: Option<String>,       // メモ
    pub account_id: String,         // どの口座からの出納かを示すID
    pub created_at: Option<String>, // 作成日時
    pub updated_at: Option<String>, // 更新日時
}
