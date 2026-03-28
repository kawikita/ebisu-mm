use crate::dao::accounts::AccountDao;
use crate::entity::accounts::{Account, AccountType};
use backtrace::Backtrace;
use serde::Deserialize;
use shaku::Component;
use sqlx::SqlitePool;
use std::fs;
use std::sync::Arc;

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

/// テスト用に口座情報を追加するヘルパー関数
/// # Arguments
/// * `pool` - Sqliteのコネクションプール
/// * `account_row` - 追加する口座情報
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

/// テスト用口座種別を挿入するヘルパー関数
/// # Arguments
/// * `pool` - Sqliteのコネクションプール
pub async fn insert_test_account(pool: &SqlitePool) {
    for account in load_accounts_from_yaml() {
        insert_account(pool, &account).await;
    }
}

/// YAMLファイルからテスト用口座情報を読み込むヘルパー関数
/// # Returns
/// * `Vec<Account>` - 読み込んだ口座情報のベクトル
pub fn load_accounts_from_yaml() -> Vec<Account> {
    let yaml_str = fs::read_to_string(TEST_ACCOUNT_DATA_PATH).expect("Failed to read YAML file");
    let yaml_accounts: Vec<YamlAccount> = serde_yaml::from_str(&yaml_str).expect("Failed to parse YAML");
    yaml_accounts
        .into_iter()
        .map(|ya| Account {
            id: ya.id,
            name: ya.name,
            account_type: AccountType {
                id: ya.account_type.id,
                name: ya.account_type.name,
            },
            memo: ya.memo,
            created_at: None,
            updated_at: None,
        })
        .collect()
}

/// テスト用口座リストを取得するヘルパー関数
/// # Returns
/// * `Vec<Account>` - 口座情報のベクトル
pub fn get_account_list() -> Vec<Account> {
    load_accounts_from_yaml()
}

/// 名前順にソートされたテスト用口座リストを取得するヘルパー関数
/// # Returns
/// * `Vec<Account>` - 名前順にソートされた口座情報のベクトル
pub fn get_sorted_account_list() -> Vec<Account> {
    let mut accounts = load_accounts_from_yaml();
    accounts.sort_by(|a, b| a.name.cmp(&b.name));
    accounts
}

/// テスト用のソート済みのリストの最初の口座を取得するヘルパー関数
/// # Returns
/// * `Account` - 最初の口座情報
pub fn get_first_account() -> Account {
    get_sorted_account_list().first().unwrap().clone()
}

/// 追加用の新しい口座データを作成するヘルパー関数
/// # Returns
/// * `Account` - 新しい口座情報
pub fn create_new_account() -> Account {
    Account {
        id: "44444444-4444-4444-4444-444444444444".to_string(),
        name: "追加の普通口座".to_string(),
        account_type: AccountType {
            id: 2,
            name: "銀行口座(普通)".to_string(),
        },
        memo: Some("追加の普通口座のメモ".to_string()),
        created_at: None,
        updated_at: None,
    }
}

// モックで呼び出し元がcreate_account関数かどうかを判定するヘルパー関数
fn is_called_from_create_account(bt: &Backtrace) -> bool {
    for frame in bt.frames() {
        for symbol in frame.symbols() {
            if let Some(name) = symbol.name() {
                if name.to_string().contains("create_account") {
                    return true;
                }
            }
        }
    }
    false
}

#[derive(Clone, Debug)]
pub struct MockConfig {
    pub error_on_create: bool,
    pub error_on_get: bool,
    pub error_on_update: bool,
    pub error_on_delete: bool,
    pub get_return_empty: bool,
    pub get_in_create_return_empty: bool,
    pub create_return_empty: bool,
    pub update_return_empty: bool,
    pub delete_return_empty: bool,
}

/// テスト用のAccountDaoモックのパラメータ化実装構造体
/// # Fields
/// * `error_on_create` - createでエラーを発生させるか
/// * `error_on_get` - getでエラーを発生させるか
/// * `error_on_update` - updateでエラーを発生させるか
/// * `error_on_delete` - deleteでエラーを発生させるか
/// * `get_return_empty` - getで空の結果を返すか
/// * `get_in_create_return_empty` - create_accountからのget_account_by_idで空の結果を返すか
/// * `create_return_empty` - createで空の結果を返すか
/// * `update_return_empty` - updateで空の結果を返すか
/// * `delete_return_empty` - deleteで空の結果を返すか
#[derive(Clone, Component)]
#[shaku(interface = AccountDao)]
pub struct ParametrizedMockAccountDaoImpl {
    pub config: Arc<MockConfig>,
}

/// テスト用のAccountDaoトレイトのモックの実装
#[async_trait::async_trait]
impl AccountDao for ParametrizedMockAccountDaoImpl {
    async fn get_accounts_list_all(&self, _pool: &SqlitePool) -> sqlx::Result<Vec<Account>> {
        if self.config.error_on_get {
            return Err(sqlx::Error::RowNotFound);
        }
        if self.config.get_return_empty {
            return Ok(vec![]);
        }
        Ok(get_sorted_account_list())
    }

    async fn get_accounts_list_by_type(&self, _pool: &SqlitePool, account_type_name: &str) -> sqlx::Result<Vec<Account>> {
        if self.config.error_on_get {
            return Err(sqlx::Error::RowNotFound);
        }
        if self.config.get_return_empty {
            return Ok(vec![]);
        }
        let accounts: Vec<Account> = get_account_list().into_iter().filter(|acc| acc.account_type.name == account_type_name).collect();
        Ok(accounts)
    }

    async fn get_account_by_id(&self, _pool: &SqlitePool, id: &str) -> sqlx::Result<Option<Account>> {
        if self.config.error_on_get {
            return Err(sqlx::Error::RowNotFound);
        }
        if self.config.get_return_empty || (self.config.get_in_create_return_empty && is_called_from_create_account(&Backtrace::new())) {
            return Ok(None);
        }
        let account = get_account_list().into_iter().find(|acc| acc.id == id);
        Ok(account)
    }

    async fn create_account(&self, _pool: &SqlitePool, _account: &Account) -> sqlx::Result<u64> {
        if self.config.error_on_create {
            return Err(sqlx::Error::RowNotFound);
        }
        if self.config.create_return_empty {
            return Ok(0);
        }
        Ok(1)
    }

    async fn update_account(&self, _pool: &SqlitePool, _account: &Account) -> sqlx::Result<u64> {
        if self.config.error_on_update {
            return Err(sqlx::Error::RowNotFound);
        }
        if self.config.update_return_empty {
            return Ok(0);
        }
        Ok(1)
    }

    async fn delete_account(&self, _pool: &SqlitePool, _id: &str) -> sqlx::Result<u64> {
        if self.config.error_on_delete {
            return Err(sqlx::Error::RowNotFound);
        }
        if self.config.delete_return_empty {
            return Ok(0);
        }
        Ok(1)
    }
}
