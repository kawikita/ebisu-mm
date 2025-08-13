-- ゑびす(Ebisu)のテーブル
-- 口座管理テーブル
CREATE TABLE accounts (
    id INTEGER PRIMARY KEY AUTOINCREMENT, -- 口座のユニークな識別子
    name TEXT NOT NULL,                  -- 口座名（例: ゆうちょ銀行、楽天カード）
    account_type TEXT NOT NULL,          -- 口座のタイプ（例: 'bank' or 'credit_card'）
    memo TEXT                            -- 口座に関する追加のメモ
);

-- 出納帳テーブル
CREATE TABLE entries (
    id INTEGER PRIMARY KEY AUTOINCREMENT, -- 取引項目のユニークな識別子
    entry_date TEXT NOT NULL,             -- 取引が行われた日付
    entry_type_id INTEGER NOT NULL,       -- どの出納タイプかを示すID
    amount INTEGER NOT NULL,              -- 取引金額
    memo TEXT,                            -- 支払先や詳細など
    account_id INTEGER NOT NULL,          -- どの口座からの取引かを示すID
    FOREIGN KEY(entry_type_id) REFERENCES entry_types(id),
    FOREIGN KEY(account_id) REFERENCES accounts(id)
);

-- 出納タイプテーブル
CREATE TABLE entry_types (
    id INTEGER PRIMARY KEY AUTOINCREMENT, -- 出納タイプのユニークな識別子
    type_name TEXT NOT NULL UNIQUE        -- 出納タイプ名（例: 入金, 出金, クレジットカード払い(一括)）
);

-- クレジットカード支払方法管理テーブル
CREATE TABLE credit_card_payment_methods (
    id INTEGER PRIMARY KEY AUTOINCREMENT, -- 支払い方法のユニークな識別子
    credit_card_account_id INTEGER NOT NULL UNIQUE, -- 紐づけるクレジットカード口座のID
    payment_bank_account_id INTEGER NOT NULL,     -- 支払い元の銀行口座のID
    closing_day INTEGER NOT NULL,                 -- 毎月の締め日
    withdrawal_day INTEGER NOT NULL,              -- 毎月の引き落とし日
    revolving_type TEXT,                          -- リボ払いの方式（例: 'standard', 'minimum'）
    revolving_interest_rate REAL,                 -- リボ払いの金利
    credit_limit INTEGER,                         -- 利用限度額
    FOREIGN KEY(credit_card_account_id) REFERENCES accounts(id),
    FOREIGN KEY(payment_bank_account_id) REFERENCES accounts(id)
);

-- クレジットカード分割払い履歴テーブル
CREATE TABLE credit_card_installments (
    id INTEGER PRIMARY KEY AUTOINCREMENT, -- 分割払い情報のユニークな識別子
    credit_card_account_id INTEGER NOT NULL, -- 関連するクレジットカード口座のID
    amount INTEGER NOT NULL,              -- 分割払いの利用金額
    installments_count INTEGER NOT NULL,  -- 分割回数
    interest_rate REAL,                   -- 分割払いの金利
    total_amount INTEGER NOT NULL,        -- 支払い総額
    first_payment INTEGER NOT NULL,       -- 初回の支払い額
    subsequent_payment INTEGER NOT NULL,  -- 2回目以降の支払い額
    paid_installments_count INTEGER NOT NULL DEFAULT 0, -- 支払済みの回数
    status TEXT NOT NULL,                 -- 支払いのステータス（例: 'pending', 'in_progress', 'completed'）
    FOREIGN KEY(credit_card_account_id) REFERENCES accounts(id)
);

-- クレジットカードリボ払い履歴テーブル
CREATE TABLE credit_card_revolving_history (
    id INTEGER PRIMARY KEY AUTOINCREMENT, -- リボ払い履歴のユニークな識別子
    credit_card_account_id INTEGER NOT NULL, -- 関連するクレジットカード口座のID
    closing_date TEXT NOT NULL,              -- 締日
    status TEXT NOT NULL,                    -- 履歴のステータス（例: 'pending_payment', 'paid'）
    amount_used INTEGER NOT NULL,            -- 前月締日後からの利用金額
    payment_amount INTEGER NOT NULL,         -- 締日の支払い額
    fee INTEGER NOT NULL,                    -- 手数料
    remaining_balance INTEGER NOT NULL,      -- 支払い後の残高
    FOREIGN KEY(credit_card_account_id) REFERENCES accounts(id)
);
