-- 口座のタイプ初期データ
INSERT INTO account_types (id, type_name) VALUES
    (1, '現金'),
    (2, '銀行口座(普通)'),
    (3, '銀行口座(定期)'),
    (4, '銀行口座(当座)'),
    (5, 'クレジットカード'),
    (6, '電子マネー'),
    (7, 'ポイント'),
    (8, '証券口座'),
    (9, '仮想通貨'),
    (10, '積立口座'),
    (99, 'その他');

-- 出納タイプ初期データ
INSERT INTO entry_types (id, type_name) VALUES
    (1, '入金'),
    (2, '出金'),
    (3, 'クレジットカード払い(一括)'),
    (4, 'クレジットカード払い(分割)'),
    (5, 'クレジットカード払い(リボ)'),
    (6, '振替'),
    (99, 'その他');

-- クレジットカード分割払いステータス初期データ
INSERT INTO credit_card_installment_status (id, status) VALUES
    (1, '支払い開始前'),
    (2, '支払い中'),
    (3, '支払い完了');

-- クレジットカードリボ払いステータス初期データ
INSERT INTO credit_card_revolving_status (id, status) VALUES
    (1, '締日前'),
    (2, '締日後');