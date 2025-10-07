#[cfg(test)]
use crate::model::accounts::{Account, AccountType};
use serde::Deserialize;
use sqlx::SqlitePool;
use std::fs;

const TEST_ACCOUNT_DATA_PATH: &str = "./tests/data/accounts.yaml";

#[derive(Debug, Deserialize)]
struct YamlAccount {
    id: String,
    name: String,
    account_type: YamlAccountType,
    memo: Option<String>,
}

#[derive(Debug, Deserialize)]
struct YamlAccountType {
    id: i64,
    name: String,
}

// テスト用に口座情報を追加するヘルパー関数
pub async fn insert_account(pool: &SqlitePool, account_row: &Account) {
    sqlx::query!(
        "INSERT INTO accounts
            (id, name, account_type_id, memo) VALUES (?1, ?2, ?3, ?4)",
        account_row.id,
        account_row.name,
        account_row.account_type.id,
        account_row.memo,
    )
    .execute(pool)
    .await
    .unwrap();
}

// テスト用口座種別を挿入するヘルパー関数
pub async fn insert_test_account(pool: &SqlitePool) {
    for account in load_accounts_from_yaml() {
        insert_account(pool, &account).await;
    }
}

// YAMLファイルからテスト用口座情報を読み込むヘルパー関数
pub fn load_accounts_from_yaml() -> Vec<Account> {
    let yaml_str = fs::read_to_string(TEST_ACCOUNT_DATA_PATH).expect("Failed to read YAML file");
    let yaml_accounts: Vec<YamlAccount> = serde_yaml::from_str(&yaml_str).expect("Failed to parse YAML");
    yaml_accounts
        .into_iter()
        .map(|ya| Account {
            id: ya.id,
            name: ya.name,
            account_type: AccountType { id: ya.account_type.id, name: ya.account_type.name },
            memo: ya.memo,
            created_at: None,
            updated_at: None,
        })
        .collect()
}

// テスト用口座リストを取得するヘルパー関数
pub fn get_account_list() -> Vec<Account> {
    load_accounts_from_yaml()
}

// 名前順にソートされたテスト用口座リストを取得するヘルパー関数
pub fn get_sorted_account_list() -> Vec<Account> {
    let mut accounts = load_accounts_from_yaml();
    accounts.sort_by(|a, b| a.name.cmp(&b.name));
    accounts
}

pub fn create_new_account() -> Account {
    Account {
        id: "44444444-4444-4444-4444-444444444444".to_string(),
        name: "追加の普通口座".to_string(),
        account_type: AccountType { id: 2, name: "銀行口座(普通)".to_string() },
        memo: Some("追加の普通口座のメモ".to_string()),
        created_at: None,
        updated_at: None,
    }
}
