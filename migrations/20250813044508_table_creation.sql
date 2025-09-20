-- ゑびす(Ebisu)のテーブル
-- 口座のタイプテーブル
DROP TABLE IF EXISTS account_types;
CREATE TABLE account_types (
    id INTEGER PRIMARY KEY AUTOINCREMENT, -- タイプのユニークな識別子
    type_name TEXT NOT NULL UNIQUE        -- タイプ名（例: 普通預金, 定期預金, クレジットカード）
);

-- 口座管理テーブル
DROP TABLE IF EXISTS accounts;
CREATE TABLE accounts (
    id TEXT PRIMARY KEY NOT NULL, -- 口座のユニークな識別子(UUID)
    name TEXT NOT NULL,                   -- 口座名（例: ゆうちょ銀行、楽天カード）
    account_type_id INTEGER NOT NULL,     -- 口座のタイプのID
    memo TEXT,                            -- 口座に関する追加のメモ
    created_at TEXT DEFAULT CURRENT_TIMESTAMP, -- 作成日時
    updated_at TEXT DEFAULT CURRENT_TIMESTAMP, -- 更新日時
    FOREIGN KEY(account_type_id) REFERENCES account_types(id)
);

-- 出納タイプテーブル
DROP TABLE IF EXISTS entry_types;
CREATE TABLE entry_types (
    id INTEGER PRIMARY KEY AUTOINCREMENT, -- 出納タイプのユニークな識別子
    type_name TEXT NOT NULL UNIQUE        -- 出納タイプ名（例: 入金, 出金, クレジットカード払い(一括)）
);

-- 出納帳テーブル
DROP TABLE IF EXISTS entries;
CREATE TABLE entries (
    entry_id TEXT PRIMARY KEY NOT NULL, -- 取引項目のユニークな識別子(UUID)
    entry_date TEXT NOT NULL,           -- 取引が行われた日付
    entry_type_id INTEGER NOT NULL,     -- どの出納タイプかを示すID
    amount INTEGER NOT NULL,            -- 取引金額
    memo TEXT,                          -- 支払先や詳細など
    account_id TEXT NOT NULL,           -- どの口座からの取引かを示すID
    created_at TEXT DEFAULT CURRENT_TIMESTAMP, -- 作成日時
    updated_at TEXT DEFAULT CURRENT_TIMESTAMP, -- 更新日時
    FOREIGN KEY(entry_type_id) REFERENCES entry_types(id),
    FOREIGN KEY(account_id) REFERENCES accounts(id)
);

-- クレジットカード支払方法管理テーブル
DROP TABLE IF EXISTS credit_card_payment_methods;
CREATE TABLE credit_card_payment_methods (
    id TEXT PRIMARY KEY NOT NULL,          -- 支払い方法のユニークな識別子(UUID)
    name TEXT NOT NULL,                    -- 支払い方法名
    credit_card_account_id TEXT NOT NULL,  -- 紐づけるクレジットカード口座のID
    payment_bank_account_id TEXT NOT NULL, -- 支払い元の銀行口座のID
    closing_day INTEGER NOT NULL,          -- 毎月の締め日
    withdrawal_day INTEGER NOT NULL,       -- 毎月の引き落とし日
    revolving_type TEXT,                   -- リボ払いの方式（例: 'standard', 'minimum'）
    revolving_interest_rate REAL,          -- リボ払いの金利
    credit_limit INTEGER,                  -- 利用限度額
    FOREIGN KEY(credit_card_account_id) REFERENCES accounts(id),
    FOREIGN KEY(payment_bank_account_id) REFERENCES accounts(id)
);

-- クレジットカード分割払いステータステーブル
DROP TABLE IF EXISTS credit_card_installment_status;
CREATE TABLE credit_card_installment_status (
    id INTEGER PRIMARY KEY AUTOINCREMENT, -- ステータスのユニークな識別子
    status TEXT NOT NULL                  -- ステータス（例: '支払い開始前', '支払い中', '支払い完了'）
);

-- クレジットカード分割払い履歴テーブル
DROP TABLE IF EXISTS credit_card_installments;
CREATE TABLE credit_card_installments (
    entry_id TEXT PRIMARY KEY NOT NULL,                 -- 出納帳のユニークな識別子(UUID)
    credit_card_account_id TEXT NOT NULL,               -- 関連するクレジットカード口座のID
    amount INTEGER NOT NULL,                            -- 分割払いの利用金額
    installments_count INTEGER NOT NULL,                -- 分割回数
    interest_rate REAL,                                 -- 分割払いの金利
    total_amount INTEGER NOT NULL,                      -- 支払い総額
    first_payment INTEGER NOT NULL,                     -- 初回の支払い額
    subsequent_payment INTEGER NOT NULL,                -- 2回目以降の支払い額
    paid_installments_count INTEGER NOT NULL DEFAULT 0, -- 支払済みの回数
    status_id INTEGER NOT NULL,                         -- 支払いのステータスのID
    usage_date TEXT NOT NULL,                           -- 利用日付
    first_payment_date TEXT NOT NULL,                   -- 初回支払い日
    last_payment_date TEXT NOT NULL,                    -- 最終支払日
    memo TEXT,                                          -- 支払先や詳 細など
    created_at TEXT DEFAULT CURRENT_TIMESTAMP, -- 作成日時
    updated_at TEXT DEFAULT CURRENT_TIMESTAMP, -- 更新日時
    FOREIGN KEY(entry_id) REFERENCES entries(entry_id),
    FOREIGN KEY(credit_card_account_id) REFERENCES accounts(id),
    FOREIGN KEY(status_id) REFERENCES credit_card_installment_status(id)
);

-- クレジットカードリボ払いステータステーブル
DROP TABLE IF EXISTS credit_card_revolving_status;
CREATE TABLE credit_card_revolving_status (
    id INTEGER PRIMARY KEY AUTOINCREMENT,  -- ステータスのユニークな識別子
    status TEXT NOT NULL                   -- ステータス（例: '締日前', '締日後'）
);

-- クレジットカードリボ払い履歴テーブル
DROP TABLE IF EXISTS credit_card_revolving_history;
CREATE TABLE credit_card_revolving_history (
    revolving_history_id TEXT PRIMARY KEY NOT NULL,     -- リボ払い履歴のユニークな識別子(UUID)
    credit_card_account_id TEXT NOT NULL,    -- 関連するクレジットカード口座のID
    closing_date TEXT NOT NULL,              -- 締日
    status_id INTEGER NOT NULL,              -- 履歴のステータスのID
    amount_used INTEGER NOT NULL,            -- 前月締日後からの利用金額
    payment_amount INTEGER NOT NULL,         -- 締日の支払い額
    fee INTEGER NOT NULL,                    -- 手数料
    remaining_balance INTEGER NOT NULL,      -- 支払い後の残高
    created_at TEXT DEFAULT CURRENT_TIMESTAMP, -- 作成日時
    updated_at TEXT DEFAULT CURRENT_TIMESTAMP, -- 更新日時
    FOREIGN KEY(credit_card_account_id) REFERENCES accounts(id),
    FOREIGN KEY(status_id) REFERENCES credit_card_revolving_status(id)
);
