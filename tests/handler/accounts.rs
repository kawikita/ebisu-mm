use crate::common::fixtures::accounts as fixtures_accounts;
use crate::common::fixtures::db as fixtures_db;
use actix_web::dev::ServiceResponse;
use actix_web::http::StatusCode;
use actix_web::{App, test, web};
use ebisu_api::entity::accounts::Account;
use ebisu_api::handler::accounts::set_route as accounts_configure;
use ebisu_api::utils::app_setup::create_app_data;
use serde_json::json;
use sqlx::{Pool, Sqlite};
use std::vec;

const API_BASE_PATH: &str = "/api/account";

enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
}

// APIを呼び出すヘルパー関数
async fn call_api(
    pool: &Pool<Sqlite>,
    method: HttpMethod,
    api_path: &str,
    send_body: Option<&Account>,
) -> ServiceResponse {
    let app_data = create_app_data(&pool);
    let app = test::init_service(App::new()
        .app_data(web::Data::new(app_data))
        .configure(accounts_configure)).await;
    let req = {
        match method {
            HttpMethod::GET => test::TestRequest::get().uri(api_path).to_request(),
            HttpMethod::POST => test::TestRequest::post().uri(api_path).set_json(send_body).to_request(),
            HttpMethod::PUT => test::TestRequest::put().uri(api_path).set_json(send_body).to_request(),
            HttpMethod::DELETE => test::TestRequest::delete().uri(api_path).to_request(),
        }
    };
    let resp = test::call_service(&app, req).await;
    resp
}

mod get_account {
    use super::*;

    #[actix_web::test]
    pub async fn get_all_is_empty() {
        // preparation
        let pool = fixtures_db::create_empty_db().await;
        // execution
        let api_path = API_BASE_PATH;
        let resp = call_api(&pool, HttpMethod::GET, &api_path, None).await;
        // assertion
        assert_eq!(resp.status(), StatusCode::OK);
        let body: Vec<Account> = test::read_body_json(resp).await;
        assert_eq!(body, vec![]);
    }

    #[actix_web::test]
    pub async fn get_all_found_3accounts() {
        // preparation
        let pool = fixtures_db::create_test_db().await;
        let accounts = fixtures_accounts::get_sorted_account_list();
        // execution
        let api_path = API_BASE_PATH;
        let resp = call_api(&pool, HttpMethod::GET, &api_path, None).await;
        // assertion
        assert_eq!(resp.status(), StatusCode::OK);
        let body: Vec<Account> = test::read_body_json(resp).await;
        assert_eq!(body.len(), 3);
        assert_eq!(body[0].id, accounts[0].id);
        assert_eq!(body[1].id, accounts[1].id);
        assert_eq!(body[2].id, accounts[2].id);
    }

    #[actix_web::test]
    pub async fn get_all_server_error() {
        let pool = fixtures_db::create_undefined_db().await;
        // execution
        let api_path = API_BASE_PATH;
        let resp = call_api(&pool, HttpMethod::GET, &api_path, None).await;
        // assertion
        assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);
        let body: serde_json::Value = test::read_body_json(resp).await;
        assert_eq!(body, json!({"error": "Failed to fetch accounts."}));
    }
}

mod get_account_type {
    use super::*;

    #[actix_web::test]
    pub async fn get_type_is_found() {
        // preparation
        let pool = fixtures_db::create_test_db().await;
        let account = fixtures_accounts::get_first_account();
        let encoded_type: String = url::form_urlencoded::byte_serialize(account.account_type.name.as_bytes()).collect();
        // execution
        let api_path = format!("{}/type/{}", API_BASE_PATH, encoded_type);
        let resp = call_api(&pool, HttpMethod::GET, &api_path, None).await;
        // assertion
        assert_eq!(resp.status(), StatusCode::OK);
        let body: Vec<Account> = test::read_body_json(resp).await;
        assert_eq!(body.len(), 1);
    }

    #[actix_web::test]
    pub async fn get_type_not_found() {
        // preparation
        let pool = fixtures_db::create_test_db().await;
        // execution
        let api_path = format!("{}/type/{}", API_BASE_PATH, "Other");
        let resp = call_api(&pool, HttpMethod::GET, &api_path, None).await;
        // assertion
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
        let body: serde_json::Value = test::read_body_json(resp).await;
        assert_eq!(body, json!({"error": "No accounts found."}));
    }

    #[actix_web::test]
    pub async fn get_type_server_error() {
        let pool = fixtures_db::create_undefined_db().await;
        let account = fixtures_accounts::get_first_account();
        let encoded_type: String = url::form_urlencoded::byte_serialize(account.account_type.name.as_bytes()).collect();
        // execution
        let api_path = format!("{}/type/{}", API_BASE_PATH, encoded_type);
        let resp = call_api(&pool, HttpMethod::GET, &api_path, None).await;
        // assertion
        assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);
        let body: serde_json::Value = test::read_body_json(resp).await;
        assert_eq!(body, json!({"error": "Failed to fetch accounts."}));
    }
}

mod get_account_id {
    use super::*;

    #[actix_web::test]
    pub async fn get_id_is_found() {
        // preparation
        let pool = fixtures_db::create_test_db().await;
        let account_id = fixtures_accounts::get_first_account().id;
        // execution
        let api_path = format!("{}/{}", API_BASE_PATH, account_id);
        let resp = call_api(&pool, HttpMethod::GET, &api_path, None).await;
        // assertion
        assert_eq!(resp.status(), StatusCode::OK);
        let body: Account = test::read_body_json(resp).await;
        assert_eq!(body.id, account_id);
    }

    #[actix_web::test]
    pub async fn get_id_not_found() {
        // preparation
        let pool = fixtures_db::create_test_db().await;
        let account_id = "55555555-5555-5555-5555-555555555555".to_string();
        // execution
        let api_path = format!("{}/{}", API_BASE_PATH, account_id);
        let resp = call_api(&pool, HttpMethod::GET, &api_path, None).await;
        // assertion
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
        let body: serde_json::Value = test::read_body_json(resp).await;
        assert_eq!(body, json!({"error": "Account not found."}));
    }

    #[actix_web::test]
    pub async fn get_id_server_error() {
        let pool = fixtures_db::create_undefined_db().await;
        let account_id = fixtures_accounts::get_first_account().id;
        // execution
        let api_path = format!("{}/{}", API_BASE_PATH, account_id);
        let resp = call_api(&pool, HttpMethod::GET, &api_path, None).await;
        // assertion
        assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);
        let body: serde_json::Value = test::read_body_json(resp).await;
        assert_eq!(body, json!({"error": "Failed to fetch account."}));
    }
}

mod post_account {
    use super::*;

    #[actix_web::test]
    pub async fn create_account_success() {
        // preparation and execution
        let pool = fixtures_db::create_empty_db().await;
        let new_account = fixtures_accounts::create_new_account();
        // execution
        let api_path = API_BASE_PATH;
        let resp = call_api(&pool, HttpMethod::POST, &api_path, Some(&new_account)).await;
        // assertion
        assert_eq!(resp.status(), StatusCode::CREATED);
        let body: serde_json::Value = test::read_body_json(resp).await;
        assert_eq!(body, json!({"created": 1}));
    }

    #[actix_web::test]
    pub async fn create_account_deprecated() {
        // preparation and execution
        let pool = fixtures_db::create_empty_db().await;
        let new_account = fixtures_accounts::create_new_account();
        // execution
        let api_path = API_BASE_PATH;
        let _ = call_api(&pool, HttpMethod::POST, &api_path, Some(&new_account)).await;
        let resp = call_api(&pool, HttpMethod::POST, &api_path, Some(&new_account)).await;
        // assertion
        assert_eq!(resp.status(), StatusCode::CONFLICT);
        let body: serde_json::Value = test::read_body_json(resp).await;
        assert_eq!(body, json!({"error": "Account ID already exists."}));
    }

    #[actix_web::test]
    pub async fn create_server_error() {
        let pool = fixtures_db::create_undefined_db().await;
        let new_account = fixtures_accounts::create_new_account();
        // execution
        let api_path = API_BASE_PATH;
        let resp = call_api(&pool, HttpMethod::POST, &api_path, Some(&new_account)).await;
        // assertion
        assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);
        let body: serde_json::Value = test::read_body_json(resp).await;
        // 重複チェックのエラーしか確認が難しい（登録時のエラーは単体テストで確認する）
        assert_eq!(body, json!({"error": "Failed to fetch account."}));
    }
}

mod delete_account {
    use super::*;

    #[actix_web::test]
    pub async fn delete_account_success() {
        // preparation
        let pool = fixtures_db::create_test_db().await;
        let account = fixtures_accounts::get_first_account();
        // execution
        let api_path = format!("{}/{}", API_BASE_PATH, account.id);
        let resp = call_api(&pool, HttpMethod::DELETE, &api_path, None).await;
        // assertion
        assert_eq!(resp.status(), StatusCode::NO_CONTENT);
    }

    #[actix_web::test]
    pub async fn delete_account_not_found() {
        // preparation
        let pool = fixtures_db::create_empty_db().await;
        fixtures_accounts::insert_test_account(&pool).await;
        let account_id = "55555555-5555-5555-5555-555555555555".to_string();
        // execution
        let api_path = format!("{}/{}", API_BASE_PATH, account_id);
        let resp = call_api(&pool, HttpMethod::DELETE, &api_path, None).await;
        // assertion
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
        let body: serde_json::Value = test::read_body_json(resp).await;
        assert_eq!(body, json!({"error": "Account not found."}));
    }

    #[actix_web::test]
    pub async fn delete_server_error() {
        let pool = fixtures_db::create_undefined_db().await;
        let account_id = fixtures_accounts::get_first_account().id;
        // execution
        let api_path = format!("{}/{}", API_BASE_PATH, account_id);
        let resp = call_api(&pool, HttpMethod::DELETE, &api_path, None).await;
        // assertion
        assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);
        let body: serde_json::Value = test::read_body_json(resp).await;
        assert_eq!(body, json!({"error": "Failed to delete account."}));
    }
}

mod put_account {
    use super::*;
    use tokio::time::{Duration, sleep};

    #[actix_web::test]
    pub async fn update_account_success() {
        // preparation
        let pool = fixtures_db::create_test_db().await;
        let before = fixtures_accounts::get_first_account();
        let before_updated_at = before.updated_at.clone();
        let mut update_account = before.clone();
        update_account.memo = Some("更新されたメモ".to_string());
        update_account.updated_at = None;
        sleep(Duration::from_secs(1)).await; // updated_atの差分を確実にするため、少し待機
        // execution
        let api_path = API_BASE_PATH;
        let resp = call_api(&pool, HttpMethod::PUT, &api_path, Some(&update_account)).await;
        // assertion
        assert_eq!(resp.status(), StatusCode::OK);
        let body: serde_json::Value = test::read_body_json(resp).await;
        assert_eq!(body, json!({"updated": 1}));
        // 更新後のデータの確認
        let updated_api_path = format!("{}/{}", API_BASE_PATH, update_account.id);
        let updated_resp = call_api(&pool, HttpMethod::GET, &updated_api_path, None).await;
        let updated_body: Account = test::read_body_json(updated_resp).await;
        assert_eq!(updated_body.id, update_account.id);
        assert_eq!(updated_body.name, update_account.name);
        assert_eq!(updated_body.account_type, update_account.account_type);
        assert_eq!(updated_body.memo, update_account.memo);
        assert_ne!(updated_body.updated_at, before_updated_at);
        assert_ne!(updated_body.created_at, updated_body.updated_at);
    }

    #[actix_web::test]
    pub async fn update_account_not_found() {
        // preparation
        let pool = fixtures_db::create_test_db().await;
        let update_account = fixtures_accounts::create_new_account();
        // execution
        let api_path = API_BASE_PATH;
        let resp = call_api(&pool, HttpMethod::PUT, &api_path, Some(&update_account)).await;
        // assertion
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
        let body: serde_json::Value = test::read_body_json(resp).await;
        assert_eq!(body, json!({"error": "Account not found."}));
    }

    #[actix_web::test]
    pub async fn update_server_error() {
        let pool = fixtures_db::create_undefined_db().await;
        let update_account = fixtures_accounts::create_new_account();
        // execution
        let api_path = API_BASE_PATH;
        let resp = call_api(&pool, HttpMethod::PUT, &api_path, Some(&update_account)).await;
        // assertion
        assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);
        let body: serde_json::Value = test::read_body_json(resp).await;
        assert_eq!(body, json!({"error": "Failed to update account."}));
    }
}
