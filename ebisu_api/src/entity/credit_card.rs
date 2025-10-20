use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// クレジットカード支払い方法の構造体
#[derive(Debug, Serialize, Deserialize, FromRow, PartialEq, PartialOrd, Clone)]
pub struct CreditCardPaymentMethod {
    pub id: String,                      // 支払い方法のユニークな識別子(UUID)
    pub name: String,                    // 支払い方法名（例: 一括払い、分割払い、リボ払い）
    pub credit_card_account_id: String,  // 紐づけるクレジットカード口座のID
    pub payment_bank_account_id: String, // 支払いに使用する銀行口座のID
    pub closing_day: i64,                // クレジットカードの締め日
    pub withdrawal_day: i64,             // クレジットカードの引き落とし日
    pub revolving_type: String,          // リボ払いの方式（例: 'standard', 'minimum'）
    pub revolving_interest_rate: f64,    // リボ払いの金利
    pub credit_limit: i64,               // 利用限度額
}

/// クレジットカード分割払いステータスの構造体
#[derive(Debug, Serialize, Deserialize, FromRow, PartialEq, Eq, Clone)]
pub struct CreditCardInstallmentStatus {
    pub id: i64,             // 分割払いステータスのユニークな識別子
    pub status_name: String, // 分割払いステータス名（例: 未払い、支払い中、完済）
}

/// クレジットカード分割払いの構造体
#[derive(Debug, Serialize, Deserialize, FromRow, PartialEq, PartialOrd, Clone)]
pub struct CreditCardInstallment {
    pub entry_id: String,               // 分割払いのユニークな識別子(UUID)
    pub credit_card_account_id: String, // 紐づけるクレジットカード口座のID
    pub amount: i64,                    // 分割払いの利用金額
    pub installments_count: i64,        // 分割回数
    pub interest_rate: f64,             // 分割払いの金利
    pub total_amount: i64,              // 支払い総額
    pub first_payment: i64,             // 初回の支払い額
    pub subsequent_payment: i64,        // 2回目以降の支払い額
    pub paid_installments_count: i64,   // 支払済みの回数
    pub status_id: i64,                 // 支払いのステータスのID
    pub usage_date: String,             // 利用日付
    pub first_payment_date: String,     // 初回支払い日
    pub last_payment_date: String,      // 最終支払日
    pub memo: Option<String>,           // 支払先や詳細など
    pub created_at: Option<String>,     // 作成日時
    pub updated_at: Option<String>,     // 更新日時
}

/// クレジットカードリボ払い状態の構造体
#[derive(Debug, Serialize, Deserialize, FromRow, PartialEq, Eq, Clone)]
pub struct CreditCardRevolvingStatus {
    pub id: i64,             // リボ払いステータスのユニークな識別子
    pub status_name: String, // リボ払いステータス名（例: 未払い、支払い中、完済）
}

/// クレジットカードリボ払い履歴の構造体
#[derive(Debug, Serialize, Deserialize, FromRow, PartialEq, PartialOrd, Clone)]
pub struct CreditCardRevolvingHistory {
    pub id: String,                 // リボ払い履歴のユニークな識別子(UUID)
    pub payment_method_id: String,  // 紐づける支払い方法のID
    pub amount: i64,                // リボ払いの利用金額
    pub status_id: i64,             // リボ払いのステータスのID
    pub usage_date: String,         // 利用日付
    pub created_at: Option<String>, // 作成日時
    pub updated_at: Option<String>, // 更新日時
}
