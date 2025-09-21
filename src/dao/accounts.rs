use crate::model::accounts::{Account, AccountType};
use sqlx::{FromRow, Result, SqlitePool};

/// 口座情報に関するデータアクセスオブジェクト(DAO)のトレイト定義。
#[trait_async::trait_async]
pub trait AccountDao {
    fn get_accounts_list_all(&self, pool: &SqlitePool) -> impl std::future::Future<Output = Result<Vec<Account>>> + Send;
    fn get_accounts_list_by_type(&self, pool: &SqlitePool, account_type_name: &str) -> impl std::future::Future<Output = Result<Vec<Account>>> + Send;
    fn get_account_by_id(&self, pool: &SqlitePool, id: &str) -> impl std::future::Future<Output = Result<Option<Account>>> + Send;
    fn create_account(&self, pool: &SqlitePool, account: &Account) -> impl std::future::Future<Output = Result<u64>> + Send;
    fn update_account(&self, pool: &SqlitePool, account: &Account) -> impl std::future::Future<Output = Result<u64>> + Send;
    fn delete_account(&self, pool: &SqlitePool, id: &str) -> impl std::future::Future<Output = Result<u64>> + Send;
}

/// AccountDaoトレイトの実装。
pub struct AccountDaoImpl;
impl AccountDao for AccountDaoImpl {
    /// すべての口座情報を取得する。
    ///
    /// # 引数
    /// * `pool` - データベース接続用のSqlitePool参照
    ///
    /// # 戻り値
    /// * `Result<Vec<Account>>` - すべての口座情報のリスト。DBエラー時はErr。
    fn get_accounts_list_all(&self, pool: &SqlitePool) -> impl std::future::Future<Output = Result<Vec<Account>>> + Send {
        async move {
            let accounts_rows = sqlx::query_as!(
                AccountRow,
                r#"
                SELECT
                T1.id as "id!: String",
                T1.name,
                T2.id as "account_type_id!: i64",
                T2.type_name as "account_type_name!",
                T1.memo,
                T1.created_at,
                T1.updated_at
                FROM accounts AS T1
                INNER JOIN account_types AS T2
                    ON T1.account_type_id = T2.id
                ORDER BY T1.name ASC
                "#,
            )
            .fetch_all(pool)
            .await?;
            Ok(convert_iter_to_accounts(accounts_rows))
        }
    }

    /// 指定した口座種別に一致する口座情報一覧を取得する。
    ///
    /// # 引数
    /// * `pool` - データベース接続用のSqlitePool参照
    /// * `account_type` - 検索対象の口座種別名
    ///
    /// # 戻り値
    /// * `Result<Vec<Account>>` - 該当する口座情報のリスト。DBエラー時はErr。
    fn get_accounts_list_by_type(&self, pool: &SqlitePool, account_type_name: &str) -> impl std::future::Future<Output = Result<Vec<Account>>> + Send {
        async move {
            let accounts_rows = sqlx::query_as!(
                AccountRow,
                r#"
                SELECT
                T1.id as "id!: String",
                T1.name,
                T2.id as "account_type_id!: i64",
                T2.type_name as "account_type_name!",
                T1.memo,
                T1.created_at,
                T1.updated_at
                FROM accounts AS T1
                INNER JOIN account_types AS T2
                    ON T1.account_type_id = T2.id
                WHERE T2.type_name = ?
                ORDER BY T1.name ASC
                "#,
                account_type_name
            )
            .fetch_all(pool)
            .await?;
            Ok(convert_iter_to_accounts(accounts_rows))
        }
    }

    /// 指定したIDの口座情報を取得する。
    ///
    /// # 引数
    /// * `pool` - データベース接続用のSqlitePool参照
    /// * `id` - 取得したい口座のID（UUID文字列）
    ///
    /// # 戻り値
    /// * `Result<Option<Account>>` - 該当口座があればSome(Account)、なければNone。DBエラー時はErr。
    fn get_account_by_id(&self, pool: &SqlitePool, id: &str) -> impl std::future::Future<Output = Result<Option<Account>>> + Send {
        async move {
            let account_row = sqlx::query_as!(
                AccountRow,
                r#"
                SELECT
                    T1.id as "id!: String",
                T1.name,
                T2.id as "account_type_id!: i64",
                T2.type_name as "account_type_name!",
                T1.memo,
                T1.created_at,
                T1.updated_at
                FROM accounts AS T1
                INNER JOIN account_types AS T2
                    ON T1.account_type_id = T2.id
                WHERE T1.id = ?
                "#,
                id
            )
            .fetch_optional(pool)
            .await?;
            Ok(account_row.map(|row| convert_row_to_object(&row)))
        }
    }

    /// 新しい口座情報を登録する。
    ///
    /// # 引数
    /// * `pool` - データベース接続用のSqlitePool参照
    /// * `account` - 登録する口座情報（Account構造体）
    ///
    /// # 戻り値
    /// * `Result<u64>` - 追加されたレコード数（通常は1）。DBエラー時はErr。
    fn create_account(&self, pool: &SqlitePool, account: &Account) -> impl std::future::Future<Output = Result<u64>> + Send {
        async move {
            let result = sqlx::query!(
                r#"
                INSERT INTO accounts (id, name, account_type_id, memo)
                VALUES (?, ?, ?, ?)
                "#,
                account.id,
                account.name,
                account.account_type.id,
                account.memo
            )
            .execute(pool)
            .await?;
            Ok(result.rows_affected())
        }
    }

    /// 既存の口座情報を更新する。
    ///
    /// # 引数
    /// * `pool` - データベース接続用のSqlitePool参照
    /// * `account` - 更新する口座情報（Account構造体）
    ///
    /// # 戻り値
    /// * `Result<u64>` - 更新されたレコード数（通常は1）。DBエラー時はErr。
    fn update_account(&self, pool: &SqlitePool, account: &Account) -> impl std::future::Future<Output = Result<u64>> + Send {
        async move {
            let account_row = convert_object_to_row(account);
            let result = sqlx::query!(
                r#"
                UPDATE accounts
                SET name = ?, account_type_id = ?, memo = ?
                WHERE id = ?
                "#,
                account_row.name,
                account_row.account_type_id,
                account_row.memo,
                account_row.id
            )
            .execute(pool)
            .await?;
            Ok(result.rows_affected())
        }
    }

    /// 指定したIDの口座情報を削除する。
    ///
    /// # 引数
    /// * `pool` - データベース接続用のSqlitePool参照
    /// * `id` - 削除したい口座のID（UUID文字列）
    ///
    /// # 戻り値
    /// * `Result<u64>` - 削除されたレコード数（通常は1）。DBエラー時はErr。
    fn delete_account(&self, pool: &SqlitePool, id: &str) -> impl std::future::Future<Output = Result<u64>> + Send {
        async move {
            let result = sqlx::query!(
                r#"
                DELETE FROM accounts
                WHERE id = ?
                "#,
            id
            )
            .execute(pool)
            .await?;
            Ok(result.rows_affected())
        }
    }
}

// プライベートヘルパー

// データベースから直接マッピングするためのヘルパー構造体
#[derive(FromRow)]
struct AccountRow {
    pub id: String,
    pub name: String,
    pub account_type_id: i64,
    pub account_type_name: String,
    pub memo: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

/// ヘルパー関数: AccountRowをAccountに変換する。
///
/// # 引数
/// * `row` - データベースから取得したAccountRow
///
/// # 戻り値
/// * `Account` - 変換後のAccount構造体
fn convert_row_to_object(row: &AccountRow) -> Account {
    Account {
        id: row.id.clone(),
        name: row.name.clone(),
        account_type: AccountType {
            id: row.account_type_id,
            name: row.account_type_name.clone(),
        },
        memo: row.memo.clone(),
        created_at: row.created_at.clone(),
        updated_at: row.updated_at.clone()
    }
}

/// ヘルパー関数: AccountをAccountRowに変換する。
///
/// # 引数
/// * `account` - 変換対象のAccount構造体
///
/// # 戻り値
/// * `AccountRow` - 変換後のAccountRow構造体
fn convert_object_to_row(account: &Account) -> AccountRow {
    AccountRow {
        id: account.id.clone(),
        name: account.name.clone(),
        account_type_id: account.account_type.id,
        account_type_name: account.account_type.name.clone(),
        memo: account.memo.clone(),
        created_at: account.created_at.clone(),
        updated_at: account.updated_at.clone()
    }
}

/// ヘルパー関数: Vec<AccountRow>をVec<Account>に変換する。
///
/// # 引数
/// * `rows` - データベースから取得したAccountRowのベクタ
///
/// # 戻り値
/// * `Vec<Account>` - 変換後のAccount構造体のベクタ
fn convert_iter_to_accounts(rows: Vec<AccountRow>) -> Vec<Account> {
    rows.into_iter()
        .map(|row| convert_row_to_object(&row))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::{SqlitePool, sqlite::SqlitePoolOptions, Executor};
    use uuid::Uuid;
    use tokio::time::{sleep, Duration};
    // テスト用のインメモリSQLiteデータベースをセットアップするヘルパー関数
    async fn create_test_db() -> SqlitePool {
        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect("file:memdb1?mode=memory&cache=shared")
            .await
            .unwrap();
        pool.execute(
            r#"
            CREATE TABLE account_types (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                type_name TEXT NOT NULL UNIQUE
            );
            CREATE TABLE accounts (
                id TEXT PRIMARY KEY NOT NULL,
                name TEXT NOT NULL,
                account_type_id INTEGER NOT NULL,
                memo TEXT,
                created_at TEXT DEFAULT CURRENT_TIMESTAMP,
                updated_at TEXT DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY(account_type_id) REFERENCES account_types(id)
            );
            CREATE TRIGGER update_accounts_updated_at
            AFTER UPDATE ON accounts
            FOR EACH ROW
            BEGIN
                UPDATE accounts
                SET updated_at = CURRENT_TIMESTAMP
                WHERE id = NEW.id;
            END;
            "#
        ).await.unwrap();
        insert_test_types(&pool).await;
        pool
    }
    // テスト用に口座種別を追加するヘルパー関数
    async fn insert_account_type(pool: &SqlitePool, type_name: &str) {
        sqlx::query!(
            "INSERT INTO account_types (type_name) VALUES (?1)",
            type_name
        )
        .execute(pool)
        .await
        .unwrap();
    }
    // テスト用に口座情報を追加するヘルパー関数
    async fn insert_account(pool: &SqlitePool, account_row: &AccountRow) {
        sqlx::query!(
            "INSERT INTO accounts
             (id, name, account_type_id, memo) VALUES (?1, ?2, ?3, ?4)",
            account_row.id,
            account_row.name,
            account_row.account_type_id,
            account_row.memo,
        )
        .execute(pool)
        .await
        .unwrap();
    }
    // テスト用口座種別を挿入するヘルパー関数
    async fn insert_test_types(pool: &SqlitePool) {
        let _ = insert_account_type(pool, "現金").await;
        let _ = insert_account_type(pool, "銀行口座(普通)").await;
        let _ = insert_account_type(pool, "銀行口座(定期)").await;
        let _ = insert_account_type(pool, "銀行口座(当座)").await;
    }
    // 普通口座のアカウント
    fn saving_account() -> AccountRow {
        AccountRow {
            id: Uuid::new_v4().to_string(),
            name: "テスト口座(普通)".to_string(),
            account_type_id: 2,
            account_type_name: "銀行口座(普通)".to_string(),
            memo: Some("テスト用の口座(普通)です".to_string()),
            created_at: None,
            updated_at: None,
        }
    }
    // 定期口座のアカウント
    fn fixed_term_account() -> AccountRow {
        AccountRow {
            id: Uuid::new_v4().to_string(),
            name: "テスト口座(定期)".to_string(),
            account_type_id: 3,
            account_type_name: "銀行口座(定期)".to_string(),
            memo: Some("テスト用の口座(定期)です".to_string()),
            created_at: None,
            updated_at: None,
        }
    }
    // 当座口座のアカウント
    fn checking_account() -> AccountRow {
        AccountRow {
            id: Uuid::new_v4().to_string(),
            name: "テスト口座(当座)".to_string(),
            account_type_id: 4,
            account_type_name: "銀行口座(当座)".to_string(),
            memo: Some("テスト用の口座(当座)です".to_string()),
            created_at: None,
            updated_at: None,
        }
    }

    // get_accounts_list_allのテスト
    mod get_accounts_list_all {
        use super::*;
        // 口座が一件もない場合
        #[tokio::test]
        async fn account_in_empty() {
            // preparation
            let pool = create_test_db().await;
            // execution
            let dao = AccountDaoImpl;
            let accounts = dao.get_accounts_list_all(&pool).await.unwrap();
            // assertion
            assert!(accounts.is_empty());
        }
        // 口座が１つ
        #[tokio::test]
        async fn account_in_one() {
            // preparation
            let saving_account = saving_account();
            let pool = create_test_db().await;
            insert_account(&pool, &saving_account).await;
            // execution
            let dao = AccountDaoImpl;
            let accounts = dao.get_accounts_list_all(&pool).await.unwrap();
            // assertion
            assert_eq!(accounts.len(), 1);
            assert_eq!(accounts[0].id, saving_account.id);
        }
        // 口座が複数件ある場合
        #[tokio::test]
        async fn account_in_two() {
            // preparation
            let saving_account = saving_account();
            let fixed_term_account = fixed_term_account();
            let pool = create_test_db().await;
            insert_account(&pool, &saving_account).await;
            insert_account(&pool, &fixed_term_account).await;
            // execution
            let dao = AccountDaoImpl;
            let accounts = dao.get_accounts_list_all(&pool).await.unwrap();
            // assertion
            assert_eq!(accounts.len(), 2);
            assert_eq!(accounts[0].id, saving_account.id);
            assert_eq!(accounts[1].id, fixed_term_account.id);
        }
    }

    // get_accounts_list_by_typeのテスト
    mod get_accounts_list_by_type {
        use super::*;
        // 該当する口座タイプが見つからない場合
        #[tokio::test]
        async fn not_found_account_type() {
            // preparation
            let pool = create_test_db().await;
            // execution
            let dao = AccountDaoImpl;
            let filtered = dao.get_accounts_list_by_type(&pool, "存在しない口座タイプ").await.unwrap();
            // assertion
            assert!(filtered.is_empty());
        }
        // 普通口座が１つ検索される
        #[tokio::test]
        async fn found_one_saving_account() {
            // preparation
            let account_type_name = "銀行口座(普通)".to_string();
            let pool = create_test_db().await;
            insert_account(&pool, &saving_account()).await;
            insert_account(&pool, &fixed_term_account()).await;
            insert_account(&pool, &checking_account()).await;
            // execution
            let dao = AccountDaoImpl;
            let filtered = dao.get_accounts_list_by_type(&pool, &account_type_name).await.unwrap();
            // assertion
            assert_eq!(filtered.len(), 1);
            assert_eq!(filtered[0].account_type.name, account_type_name);
        }
        // 普通口座２つ検索される
        #[tokio::test]
        async fn found_two_saving_accounts() {
            // preparation
            let account_type_name = "銀行口座(普通)".to_string();
            let pool = create_test_db().await;
            insert_account(&pool, &saving_account()).await;
            insert_account(&pool, &fixed_term_account()).await;
            insert_account(&pool, &checking_account()).await;
            insert_account(&pool, &saving_account()).await;
            // execution
            let dao = AccountDaoImpl;
            let filtered = dao.get_accounts_list_by_type(&pool, &account_type_name).await.unwrap();
            // assertion
            assert_eq!(filtered.len(), 2);
            assert_eq!(filtered[0].account_type.name, account_type_name);
            assert_eq!(filtered[1].account_type.name, account_type_name);
        }
    }

    // get_account_by_idのテスト
    mod get_account_by_id {
        use super::*;
        // 該当する口座が見つからない場合
        #[tokio::test]
        async fn not_found_account() {
            // preparation
            let id = Uuid::new_v4().to_string();
            let pool = create_test_db().await;
            insert_account(&pool, &saving_account()).await;
            insert_account(&pool, &fixed_term_account()).await;
            insert_account(&pool, &checking_account()).await;
            // execution
            let dao = AccountDaoImpl;
            let fetched = dao.get_account_by_id(&pool, &id).await.unwrap();
            // assertion
            assert!(fetched.is_none());
        }
        // 該当する口座が見つかった場合
        #[tokio::test]
        async fn found_account_by_id() {
            // preparation
            let saving_account = saving_account();
            let pool = create_test_db().await;
            insert_account(&pool, &saving_account).await;
            insert_account(&pool, &fixed_term_account()).await;
            insert_account(&pool, &checking_account()).await;
            // execution
            let dao = AccountDaoImpl;
            let fetched = dao.get_account_by_id(&pool, &saving_account.id).await.unwrap();
            // assertion
            assert!(fetched.is_some());
            let fetched = fetched.unwrap();
            assert_eq!(fetched.id, saving_account.id);
            assert_eq!(fetched.name, saving_account.name);
            assert_eq!(fetched.account_type.id, saving_account.account_type_id);
            assert_eq!(fetched.memo, saving_account.memo);
        }
    }

    // create_accountのテスト
    mod create_account {
        use super::*;
        //  口座を追加する
        #[tokio::test]
        async fn create_account() {
            // preparation
            let pool = create_test_db().await;
            // execution
            let dao = AccountDaoImpl;
            let saving_account = saving_account();
            let result = dao.create_account(&pool, &convert_row_to_object(&saving_account)).await.unwrap();
            // assertion
            assert_eq!(result, 1);
            let fetched = dao.get_account_by_id(&pool, &saving_account.id).await.unwrap().unwrap();
            assert_eq!(fetched.id, saving_account.id);
            assert_eq!(fetched.name, saving_account.name);
            assert_eq!(fetched.account_type.id, saving_account.account_type_id);
            assert_eq!(fetched.memo, saving_account.memo);
            assert!(!fetched.created_at.is_none());
            assert!(!fetched.updated_at.is_none());
        }
    }

    // update_accountのテスト
    mod update_account {
        use super::*;
        // 口座情報を更新する
        #[tokio::test]
        async fn updates_account() {
            // preparation
            let before_account = saving_account();
            let pool = create_test_db().await;
            insert_account(&pool, &before_account).await;
            let dao = AccountDaoImpl;
            let before_fetched = dao.get_account_by_id(&pool, &before_account.id).await.unwrap().unwrap();
            let mut updated_account = convert_object_to_row(&before_fetched);
            updated_account.name = "更新後の口座名".to_string();
            updated_account.account_type_id = 3; // 銀行口座(定期)
            updated_account.account_type_name = "銀行口座(定期)".to_string();
            updated_account.memo = Some("更新後のメモ".to_string());
            let before_updated_at = before_fetched.updated_at.clone();
            sleep(Duration::from_secs(1)).await; // updated_atの差分を確実にするため、1秒待機
            // execution
            let updated = dao.update_account(&pool, &convert_row_to_object(&updated_account)).await.unwrap();
            // assertion
            assert_eq!(updated, 1);
            let fetched = dao.get_account_by_id(&pool, &updated_account.id).await.unwrap().unwrap();
            assert_eq!(fetched.id, updated_account.id);
            assert_eq!(fetched.name, updated_account.name);
            assert_eq!(fetched.account_type.id, updated_account.account_type_id);
            assert_eq!(fetched.memo, updated_account.memo);
            assert!(!fetched.created_at.is_none());
            assert!(!fetched.updated_at.is_none());
            assert_ne!(fetched.updated_at, before_updated_at);
            assert_ne!(fetched.created_at, fetched.updated_at);
        }
    }

    mod delete_account {
        use super::*;

        // 口座が削除される
        #[tokio::test]
        async fn deletes_account() {
            // preparation
            let saving_account = saving_account();
            let pool = create_test_db().await;
            insert_account(&pool, &saving_account).await;
            insert_account(&pool, &fixed_term_account()).await;
            insert_account(&pool, &checking_account()).await;
            // execution
            let dao = AccountDaoImpl;
            let deleted = dao.delete_account(&pool, &saving_account.id).await.unwrap();
            // assertion
            assert_eq!(deleted, 1);
            let fetched = dao.get_account_by_id(&pool, &saving_account.id).await.unwrap();
            assert!(fetched.is_none());
        }
    }
}