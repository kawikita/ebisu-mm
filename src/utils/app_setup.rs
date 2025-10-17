use crate::dao::accounts::AccountDaoImpl;
use crate::handler::accounts::{self, AccountHandlerImpl};
use crate::handler::swagger_ui;
use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use log::{debug, info};
use serde_json::json;
use shaku::module;
use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};
use std::env;

const DEFAULT_DATABASE_URL: &str = "sqlite:./ebisu.db";
const DEFAULT_MAX_CONNECTIONS: &str = "5";
const DEFAULT_SERVER_ADDRESS: &str = "127.0.0.1";
const DEFAULT_SERVER_PORT: &str = "8180";

// Shakuのモジュール定義
module! {
    pub AppModule {
        components = [AccountDaoImpl, AccountHandlerImpl],
        providers = []
    }
}

/// ルートエントリーポイントのルーティング設定関数
pub fn set_route(cfg: &mut web::ServiceConfig) {
    debug!("Setting up root route configuration.");
    cfg.service(web::resource("/").route(web::get().to(index)));
    debug!("Root route configuration set up successfully.");
}

/// 各APIのルーティング設定をする関数
fn set_route_config(cfg: &mut web::ServiceConfig) {
    debug!("Setting up route configurations.");
    set_route(cfg);
    accounts::set_route(cfg);
    swagger_ui::set_route(cfg);
    debug!("Route configurations set up successfully.");
}

/// Actix Webアプリケーションのファクトリ関数
pub async fn get_server() -> actix_web::dev::Server {
    info!("Configuring and starting Actix Web server.");
    let db_pool = web::Data::new(get_db_pool().await);
    let app_module = web::Data::new(AppModule::builder().build());
    let server = HttpServer::new(move || {
        App::new().app_data(db_pool.clone()).app_data(app_module.clone()).configure(set_route_config)
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
