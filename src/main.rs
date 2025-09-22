// Ebisu backend
// Copyright (c) 2025 Samurai QA Laboratory
use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use dotenv::dotenv;
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use std::env;
use log::info;

struct AppState {
    db: SqlitePool,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    ebisu_api::utils::logging::init_logger();
    info!("Starting Ebisu API server...");

    let database_url: String = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create database connectionpool");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState { db: pool.clone() }))
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Welcome to Ebisu API!")
}