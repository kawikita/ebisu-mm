// Ebisu backend
// Copyright (c) 2025 Samurai QA Laboratory
// This software is released under the MIT License.

use dotenv::dotenv;
use ebisu_api::utils::{app_setup, logging};
use log::info;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    logging::init_logger();
    info!("Welcome to Ebisu API!");
    info!("Starting Ebisu API server...");
    let pool = app_setup::get_db_pool().await;
    let app_data = app_setup::create_app_data(&pool);
    let server = app_setup::get_server(app_data).await;
    info!("Server is running. Press Ctrl+C to stop.");
    let result = server.await;
    info!("Ebisu API server stopped.");
    result
}
