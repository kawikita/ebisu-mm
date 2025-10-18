#[cfg(test)]
use sqlx::{Executor, SqlitePool, sqlite::SqlitePoolOptions};
use std::fs;
use uuid::Uuid;

const MIGRATIONS_DIR: &str = "./migrations";

// テスト用のインメモリSQLiteデータベースをセットアップするヘルパー関数
pub async fn create_test_db() -> SqlitePool {
    let db_name = format!("file:memdb-{}", Uuid::new_v4().to_string());
    let db_url = format!("{}?mode=memory&cache=shared", db_name);
    let pool = SqlitePoolOptions::new().max_connections(1).connect(&db_url).await.unwrap();
    let mut conn = pool.acquire().await.unwrap();
    // マイグレーションファイルを読み込み、順番に実行する
    let mut migration_paths: Vec<_> = fs::read_dir(MIGRATIONS_DIR)
        .expect("Failed to read migrations directory")
        .filter_map(|entry| {
            let path = entry.ok()?.path();
            if path.extension().map_or(false, |ext| ext == "sql") { Some(path) } else { None }
        })
        .collect();
    migration_paths.sort();
    for path in migration_paths {
        let sql_content = fs::read_to_string(&path).expect(&format!("Failed to read SQL file: {}", path.display()));
        conn.execute(sql_content.as_str())
            .await
            .unwrap_or_else(|e| panic!("Failed to execute migration: {}: {}", path.display(), e));
    }
    pool
}

// スキーマが設定されていないDBへのアクセスプールを作成する
pub async fn create_undefined_db() -> SqlitePool {
    let db_name = format!("file:memdb-{}", Uuid::new_v4().to_string());
    let db_url = format!("{}?mode=memory&cache=shared", db_name);
    let pool = SqlitePoolOptions::new().max_connections(1).connect(&db_url).await.unwrap();
    pool
}
