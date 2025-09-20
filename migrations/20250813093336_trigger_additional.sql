-- トリガーの作成

-- accountsテーブルに挿入時にupdated_atとcreated_atを自動更新するトリガーを作成
CREATE TRIGGER insert_accounts_timestamps
AFTER INSERT ON accounts
FOR EACH ROW
BEGIN
    UPDATE accounts
    SET created_at = CURRENT_TIMESTAMP,
        updated_at = CURRENT_TIMESTAMP
    WHERE id = NEW.id;
END;

-- accountsテーブルの更新時にupdated_atを自動更新するトリガーを作成
CREATE TRIGGER update_accounts_updated_at
AFTER UPDATE ON accounts
FOR EACH ROW
BEGIN
    UPDATE accounts
    SET updated_at = CURRENT_TIMESTAMP
    WHERE id = NEW.id;
END;

-- entriesテーブルの更新時にupdated_atを自動更新するトリガーを作成
CREATE TRIGGER update_entries_updated_at
AFTER UPDATE ON entries
FOR EACH ROW
BEGIN
    UPDATE entries
    SET updated_at = CURRENT_TIMESTAMP
    WHERE entry_id = NEW.entry_id;
END;

-- credit_card_installmentsテーブルの更新時にupdated_atを自動更新するトリガーを作成
CREATE TRIGGER update_credit_card_installments_updated_at
AFTER UPDATE ON credit_card_installments
FOR EACH ROW
BEGIN
    UPDATE credit_card_installments
    SET updated_at = CURRENT_TIMESTAMP
    WHERE entry_id = NEW.entry_id;
END;

-- credit_card_revolving_historyテーブルの更新時にupdated_atを自動更新するトリガーを作成
CREATE TRIGGER update_credit_card_revolving_history_updated_at
AFTER UPDATE ON credit_card_revolving_history
FOR EACH ROW
BEGIN
    UPDATE credit_card_revolving_history
    SET updated_at = CURRENT_TIMESTAMP
    WHERE revolving_history_id = NEW.revolving_history_id;
END;