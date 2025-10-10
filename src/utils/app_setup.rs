use crate::dao::accounts::AccountDaoImpl;
use crate::handler::accounts;
use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use log::{debug, info};
use serde_json::json;
use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};
use std::env;

const DEFAULT_DATABASE_URL: &str = "sqlite:./ebisu.db";
const DEFAULT_MAX_CONNECTIONS: &str = "5";
const DEFAULT_SERVER_ADDRESS: &str = "127.0.0.1";
const DEFAULT_SERVER_PORT: &str = "8180";

// DAOインスタンスをセットするための構造体
#[derive(Clone)]
pub struct AppData {
    pub db_pool: SqlitePool,
    pub account_dao: AccountDaoImpl,
}

/// DAOのインスタンスを作成してAppDataにセットする関数
pub fn create_app_data(pool: &SqlitePool) -> AppData {
    AppData { db_pool: pool.clone(), account_dao: AccountDaoImpl }
}

/// 各APIのルーティング設定をする関数
fn set_route_config(cfg: &mut web::ServiceConfig) {
    debug!("Setting up route configurations.");
    accounts::set_route(cfg);
    debug!("Route configurations set up successfully.");
}

/// Actix Webアプリケーションのファクトリ関数
pub async fn get_server(app_data: AppData) -> actix_web::dev::Server {
    info!("Configuring and starting Actix Web server.");
    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_data.clone()))
            .configure(set_route_config)
            .route("/", web::get().to(index))
    })
    .bind(get_server_and_port())
    .expect("Failed to bind server address")
    .run();
    info!("Actix Web server configured and started successfully.");
    server
}

// ルートエントリーポイントのハンドラー関数
async fn index() -> impl Responder {
    HttpResponse::Ok().json(json!({"message": "Welcome to Ebisu API!"}))
}

// サーバーのアドレスとポートを環境変数から取得する関数
fn get_server_and_port() -> String {
    info!("Fetching server address and port from environment variables.");
    let server = env::var("SERVER_ADDRESS").unwrap_or_else(|_| DEFAULT_SERVER_ADDRESS.into());
    let port = env::var("SERVER_PORT").unwrap_or_else(|_| DEFAULT_SERVER_PORT.into());
    let address_and_port = format!("{}:{}", server, port);
    info!("Server will bind to address: {}", address_and_port);
    address_and_port
}

// データベース接続プールを取得する関数
pub async fn get_db_pool() -> SqlitePool {
    info!("Creating database connection pool");
    let database_url: String = env::var("DATABASE_URL").unwrap_or_else(|_| DEFAULT_DATABASE_URL.into());
    let max_connections: u32 = env::var("DATABASE_MAX_CONNECTIONS")
        .unwrap_or_else(|_| DEFAULT_MAX_CONNECTIONS.into())
        .parse()
        .expect("DATABASE_MAX_CONNECTIONS must be a valid number");
    let pool = SqlitePoolOptions::new()
        .max_connections(max_connections)
        .connect(&database_url)
        .await
        .expect("Failed to create database connection pool");
    info!("Database connection pool created successfully.");
    info!("  Database URL: {}", database_url);
    info!("  Max connections: {}", max_connections);
    pool
}
