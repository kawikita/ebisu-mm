use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

/// 口座種別を表す構造体。
#[derive(Debug, Serialize, Deserialize, FromRow, PartialEq, Eq, Clone, ToSchema)]
pub struct AccountType {
    /// [必須]口座種別のユニークな識別子
    pub id: i64,
    /// [必須]口座種別名
    pub name: String,
}

/// 口座情報を表す構造体。accountsテーブルの1レコードに対応。
#[derive(Debug, Serialize, Deserialize, FromRow, PartialEq, Eq, Clone, ToSchema)]
pub struct Account {
    /// [必須]口座のユニークな識別子(UUID)
    pub id: String,
    /// [必須]口座名
    pub name: String,
    /// [必須]口座種別
    pub account_type: AccountType,
    /// [任意]メモ
    pub memo: Option<String>,
    /// [任意]作成日時(ISO 8601形式)
    pub created_at: Option<String>,
    /// [任意]更新日時(ISO 8601形式)
    pub updated_at: Option<String>,
}
