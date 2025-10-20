use crate::fixtures::accounts;
use sqlx::{Executor, SqlitePool, sqlite::SqlitePoolOptions};
use std::fs;
use uuid::Uuid;

const MIGRATIONS_DIR: &str = "../ebisu_api/migrations";

/// テスト用のインメモリSQLiteデータベースをセットアップするヘルパー関数
/// # Returns:
///   SqlitePool - マイグレーションが適用され、テストデータが挿入されたSQLite接続プール
/// # Panics
///   マイグレーションの適用に失敗した場合、パニックします。
pub async fn create_test_db() -> SqlitePool {
    let pool = create_empty_db().await;
    accounts::insert_test_account(&pool).await;
    pool
}

/// 空のデータベースを作成する
/// # Returns:
///   SqlitePool - マイグレーションが適用されたSQLite接続プール
/// # Panics
///   マイグレーションの適用に失敗した場合、パニックします。
pub async fn create_empty_db() -> SqlitePool {
    let pool = create_undefined_db().await;
    migrate_test_db(&pool).await;
    pool
}

/// スキーマが設定されていないDBへのアクセスプールを作成する
/// # Returns:
///   SqlitePool - スキーマが設定されていないSQLite接続プール
/// # Panics
///   接続プールの作成に失敗した場合、パニックします。
pub async fn create_undefined_db() -> SqlitePool {
    let db_name = format!("file:memdb-{}", Uuid::new_v4().to_string());
    let db_url = format!("{}?mode=memory&cache=shared", db_name);
    let pool = SqlitePoolOptions::new().max_connections(1).connect(&db_url).await.unwrap();
    pool
}

/// SQLスキーマやトリガーなどのマイグレーションを適用するヘルパー関数
/// # Arguments
/// * `pool` - Sqliteのコネクションプール
/// # Panics
///   マイグレーションの適用に失敗した場合、パニックします。
pub async fn migrate_test_db(pool: &SqlitePool) {
    let mut conn = pool.acquire().await.unwrap();
    // マイグレーションファイルを読み込み、順番に実行する
    let paths = fs::read_dir(MIGRATIONS_DIR).expect("Failed to read migrations directory");
    for entry in paths {
        let path = entry.expect("Failed to get entry").path();
        if path.extension().map_or(false, |ext| ext == "sql") {
            let sql_content = fs::read_to_string(&path).unwrap_or_else(|e|panic!("Failed to read SQL file {}: {}", path.display(), e));
            conn.execute(sql_content.as_str())
                .await
                .unwrap_or_else(|e| panic!("Failed to execute migration {}: {}", path.display(), e));
        }
    }
}
