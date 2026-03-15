use crate::utils::unit_test::fixtures::accounts;
use sqlx::{Executor, SqlitePool, sqlite::SqlitePoolOptions};
use std::fs;
use uuid::Uuid;

const MIGRATIONS_DIR: &str = "./migrations";

/// テスト用のインメモリSQLiteデータベースをセットアップするヘルパー関数

// 空のデータベースを作成する
pub async fn create_empty_db() -> SqlitePool {
    db_migration().await
}

// テストデータが入ったデータベースを作成する
pub async fn create_test_db() -> SqlitePool {
    let pool = create_empty_db().await;
    accounts::insert_test_account(&pool).await;
    pool
}

// スキーマが設定されていないDBへのアクセスプールを作成する
pub async fn create_undefined_db() -> SqlitePool {
    let db_name = format!("file:memdb-{}", Uuid::new_v4().to_string());
    let db_url = format!("{}?mode=memory&cache=shared", db_name);
    let pool = SqlitePoolOptions::new().connect(&db_url).await.unwrap();
    pool
}

// マイグレーションを適用したテスト用データベースを作成する
pub async fn db_migration() -> SqlitePool {
    let pool = create_undefined_db().await;
    let mut conn = pool.acquire().await.unwrap();
    // マイグレーションファイルを読み込み、順番に実行する
    for sql_path in collect_sqlfiles() {
        let sql_content =
            fs::read_to_string(&sql_path).expect(&format!("Failed to read SQL file: {}", sql_path.display()));
        conn.execute(sql_content.as_str())
            .await
            .expect(&format!("Failed to execute migration: {}", sql_path.display()));
    }
    pool
}

fn collect_sqlfiles() -> Vec<std::path::PathBuf> {
    let mut sql_paths: Vec<std::path::PathBuf> = fs::read_dir(MIGRATIONS_DIR)
        .expect("Failed to read migrations directory")
        .filter_map(|entry| {
            let path = entry.ok()?.path();
            if path.extension().map_or(false, |ext| ext == "sql") { Some(path) } else { None }
        })
        .collect();
    sql_paths.sort_by(|a, b| a.file_name().cmp(&b.file_name()));
    sql_paths
}
