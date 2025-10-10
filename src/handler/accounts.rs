use crate::dao::accounts::{AccountDao, AccountDaoImpl};
use crate::model::accounts::Account;
use actix_web::{HttpResponse, Result, web};
use log::{error, info};
use serde_json::json;
use sqlx::SqlitePool;

const API_BASE_PATH: &str = "/api/account";

/// APIのルーティング設定
/// # 引数
/// * `cfg` - Actix WebのServiceConfigオブジェクト
/// # 戻り値
/// なし
pub fn set_route(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope(API_BASE_PATH)
            .service(
                web::resource("")
                    .route(web::get().to(
                        |handler: web::Data<AccountHandlerImpl>,
                         pool: web::Data<SqlitePool>,
                         dao: web::Data<AccountDaoImpl>| async move {
                            handler.get_accounts_list_all(pool, dao).await
                        },
                    ))
                    .route(web::post().to(
                        |handler: web::Data<AccountHandlerImpl>,
                         pool: web::Data<SqlitePool>,
                         dao: web::Data<AccountDaoImpl>,
                         data: web::Json<Account>| async move {
                            handler.create_account(pool, dao, data).await
                        },
                    ))
                    .route(web::put().to(
                        |handler: web::Data<AccountHandlerImpl>,
                         pool: web::Data<SqlitePool>,
                         dao: web::Data<AccountDaoImpl>,
                         data: web::Json<Account>| async move {
                            handler.update_account(pool, dao, data).await
                        },
                    )),
            )
            .service(
                web::resource("/{id}")
                    .route(web::get().to(
                        |handler: web::Data<AccountHandlerImpl>,
                         pool: web::Data<SqlitePool>,
                         dao: web::Data<AccountDaoImpl>,
                         path: web::Path<String>| async move {
                            handler.get_account_by_id(pool, dao, path).await
                        },
                    ))
                    .route(web::delete().to(
                        |handler: web::Data<AccountHandlerImpl>,
                         pool: web::Data<SqlitePool>,
                         dao: web::Data<AccountDaoImpl>,
                         path: web::Path<String>| async move {
                            handler.delete_account(pool, dao, path).await
                        },
                    )),
            )
            .service(web::resource("/type/{type_name}").route(web::get().to(
                |handler: web::Data<AccountHandlerImpl>,
                 pool: web::Data<SqlitePool>,
                 dao: web::Data<AccountDaoImpl>,
                 path: web::Path<String>| async move {
                    handler.get_accounts_list_by_type(pool, dao, path).await
                },
            ))),
    );
}

// ハンドラートレイト定義（グローバルスコープに移動）
#[async_trait::async_trait]
pub trait AccountHandler {
    async fn create_account(
        &self,
        pool: web::Data<SqlitePool>,
        dao: web::Data<AccountDaoImpl>,
        data: web::Json<Account>,
    ) -> Result<HttpResponse, actix_web::Error>;
    async fn delete_account(
        &self,
        pool: web::Data<SqlitePool>,
        dao: web::Data<AccountDaoImpl>,
        path: web::Path<String>,
    ) -> Result<HttpResponse, actix_web::Error>;
    async fn get_account_by_id(
        &self,
        pool: web::Data<SqlitePool>,
        dao: web::Data<AccountDaoImpl>,
        path: web::Path<String>,
    ) -> Result<HttpResponse, actix_web::Error>;
    async fn get_accounts_list_all(
        &self,
        pool: web::Data<SqlitePool>,
        dao: web::Data<AccountDaoImpl>,
    ) -> Result<HttpResponse, actix_web::Error>;
    async fn get_accounts_list_by_type(
        &self,
        pool: web::Data<SqlitePool>,
        dao: web::Data<AccountDaoImpl>,
        path: web::Path<String>,
    ) -> Result<HttpResponse, actix_web::Error>;
    async fn update_account(
        &self,
        pool: web::Data<SqlitePool>,
        dao: web::Data<AccountDaoImpl>,
        data: web::Json<Account>,
    ) -> Result<HttpResponse, actix_web::Error>;
}

/// 口座情報ハンドラーの実装
#[derive(Clone)]
pub struct AccountHandlerImpl;

#[async_trait::async_trait]
impl AccountHandler for AccountHandlerImpl {
    /// 新しい口座情報を作成するハンドラー関数
    /// # 引数
    /// * `pool` - データベース接続プール
    /// * `dao` - 口座情報DAO
    /// * `data` - リクエストボディから取得した新しい口座情報
    /// # 戻り値
    /// 成功時はHTTP 201と作成された口座情報のJSON、失敗時はHTTP 500とエラーメッセージ
    async fn create_account(
        &self,
        pool: web::Data<SqlitePool>,
        dao: web::Data<AccountDaoImpl>,
        data: web::Json<Account>,
    ) -> Result<HttpResponse, actix_web::Error> {
        info!("Received request to create a new account");
        let account_id = data.id.clone();
        // IDの重複チェック
        let result = dao.get_account_by_id(&pool, &account_id).await;
        match result {
            Ok(account) => {
                if account.is_some() {
                    info!("Account with ID {} already exists.", account_id);
                    return Ok(HttpResponse::Conflict().json(json!({"error": "Account ID already exists."})));
                }
            },
            Err(e) => {
                error!("Database error: {:?}", e);
                return Ok(HttpResponse::InternalServerError().json(json!({"error": "Failed to fetch account."})));
            },
        }
        // 口座の作成
        let result = dao.create_account(&pool, &data).await;
        match result {
            Ok(success_count) => {
                info!("Created {} new account(s).", success_count);
                Ok(HttpResponse::Created().json(json!({"created": success_count})))
            },
            Err(e) => {
                error!("Database error: {:?}", e);
                Ok(HttpResponse::InternalServerError().json(json!({"error": "Failed to create account."})))
            },
        }
    }

    /// 指定されたIDの口座情報を削除するハンドラー関数
    /// # 引数
    /// * `pool` - データベース接続プール
    /// * `dao` - 口座情報DAO
    /// * `path` - URLパスから取得した口座ID
    /// # 戻り値
    /// 成功時はHTTP 200と成功メッセージ、失敗時はHTTP 500とエラーメッセージ
    async fn delete_account(
        &self,
        pool: web::Data<SqlitePool>,
        dao: web::Data<AccountDaoImpl>,
        path: web::Path<String>,
    ) -> Result<HttpResponse, actix_web::Error> {
        let account_id = path.as_str();
        info!("Received request to delete account by ID: {}", account_id);
        let result = dao.delete_account(&pool, account_id).await;
        match result {
            Ok(del_count) => {
                if del_count == 0 {
                    info!("No account found with ID: {}", account_id);
                    return Ok(HttpResponse::NotFound().json(json!({"error": "Account not found."})));
                }
                info!("Deleted account with ID: {}", account_id);
                Ok(HttpResponse::NoContent().json(json!({"message": "Account deleted successfully."})))
            },
            Err(e) => {
                error!("Database error: {:?}", e);
                Ok(HttpResponse::InternalServerError().json(json!({"error": "Failed to delete account."})))
            },
        }
    }

    /// 指定されたIDの口座情報を取得するハンドラー関数
    /// # 引数
    /// * `pool` - データベース接続プール
    /// * `dao` - 口座情報DAO
    /// * `path` - URLパスから取得した口座ID
    /// # 戻り値
    /// 成功時はHTTP 200と口座情報のJSON、失敗時はHTTP 500とエラーメッセージ
    async fn get_account_by_id(
        &self,
        pool: web::Data<SqlitePool>,
        dao: web::Data<AccountDaoImpl>,
        path: web::Path<String>,
    ) -> Result<HttpResponse, actix_web::Error> {
        let account_id = path.as_str();
        info!("Received request to fetch account by ID: {}", account_id);
        let result = dao.get_account_by_id(&pool, account_id).await;
        match result {
            Ok(account) => {
                if account.is_none() {
                    info!("No account found with ID: {}", account_id);
                    return Ok(HttpResponse::NotFound().json(json!({"error": "Account not found."})));
                }
                info!("Fetched account with ID: {}", account_id);
                Ok(HttpResponse::Ok().json(account))
            },
            Err(e) => {
                error!("Database error: {:?}", e);
                Ok(HttpResponse::InternalServerError().json(json!({"error": "Failed to fetch account."})))
            },
        }
    }

    /// 全ての口座情報を取得するハンドラー関数
    /// # 引数
    /// * `pool` - データベース接続プール
    /// * `dao` - 口座情報DAO
    /// # 戻り値
    /// 成功時はHTTP 200と口座情報のJSON配列、失敗時はHTTP 500とエラーメッセージ
    async fn get_accounts_list_all(
        &self,
        pool: web::Data<SqlitePool>,
        dao: web::Data<AccountDaoImpl>,
    ) -> Result<HttpResponse, actix_web::Error> {
        info!("Received request to fetch all accounts");
        let result = dao.get_accounts_list_all(&pool).await;
        match result {
            Ok(accounts) => {
                info!("Fetched {} accounts.", accounts.len());
                Ok(HttpResponse::Ok().json(accounts))
            },
            Err(e) => {
                error!("Database error: {:?}", e);
                Ok(HttpResponse::InternalServerError().json(json!({"error": "Failed to fetch accounts."})))
            },
        }
    }

    /// 指定された口座種別の口座情報を取得するハンドラー関数
    /// # 引数
    /// * `pool` - データベース接続プール
    /// * `dao` - 口座情報DAO
    /// * `path` - URLパスから取得した口座種別名
    /// # 戻り値
    /// 成功時はHTTP 200と口座情報のJSON配列、失敗時はHTTP 500とエラーメッセージ
    async fn get_accounts_list_by_type(
        &self,
        pool: web::Data<SqlitePool>,
        dao: web::Data<AccountDaoImpl>,
        path: web::Path<String>,
    ) -> Result<HttpResponse, actix_web::Error> {
        let type_name = path.as_str();
        info!("Received request to fetch accounts of type: {}", type_name);
        let result = dao.get_accounts_list_by_type(&pool, type_name).await;
        match result {
            Ok(accounts) => {
                if accounts.is_empty() {
                    info!("No accounts found for type: {}", type_name);
                    return Ok(HttpResponse::NotFound().json(json!({"error": "No accounts found."})));
                }
                info!("Fetched {} accounts of type: {}", accounts.len(), type_name);
                Ok(HttpResponse::Ok().json(accounts))
            },
            Err(e) => {
                error!("Database error: {:?}", e);
                Ok(HttpResponse::InternalServerError().json(json!({"error": "Failed to fetch accounts."})))
            },
        }
    }

    /// 指定されたIDの口座情報を更新するハンドラー関数
    /// # 引数
    /// * `pool` - データベース接続プール
    /// * `dao` - 口座情報DAO
    /// * `data` - リクエストボディから取得した更新後の口座情報
    /// # 戻り値
    /// 成功時はHTTP 200と更新された口座情報のJSON、失敗時はHTTP 500とエラーメッセージ
    async fn update_account(
        &self,
        pool: web::Data<SqlitePool>,
        dao: web::Data<AccountDaoImpl>,
        data: web::Json<Account>,
    ) -> Result<HttpResponse, actix_web::Error> {
        info!("Received request to update account by ID: {}", data.id);
        let result = dao.update_account(&pool, &data).await;
        match result {
            Ok(updated_count) => {
                if updated_count == 0 {
                    info!("No account found with ID: {}", data.id);
                    return Ok(HttpResponse::NotFound().json(json!({"error": "Account not found."})));
                }
                info!("Updated account with ID: {}", data.id);
                Ok(HttpResponse::Ok().json(json!({"updated": updated_count})))
            },
            Err(e) => {
                error!("Database error: {:?}", e);
                Ok(HttpResponse::InternalServerError().json(json!({"error": "Failed to update account."})))
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::unit_test::fixtures::accounts as fixtures_accounts;
    use crate::utils::unit_test::fixtures::db as fixtures_db;
    use actix_web::body::to_bytes;
    use actix_web::http::StatusCode;

    async fn setup() -> (web::Data<SqlitePool>, web::Data<AccountDaoImpl>, AccountHandlerImpl) {
        let (pool, dao, handler) = setup_empty().await;
        fixtures_accounts::insert_test_account(&pool).await;
        (pool, dao, handler)
    }

    async fn setup_empty() -> (web::Data<SqlitePool>, web::Data<AccountDaoImpl>, AccountHandlerImpl) {
        let pool: web::Data<sqlx::Pool<sqlx::Sqlite>> = web::Data::new(fixtures_db::create_test_db().await);
        let (dao, handler) = get_dao_and_handler();
        (pool, dao, handler)
    }

    async fn setup_undefined() -> (web::Data<SqlitePool>, web::Data<AccountDaoImpl>, AccountHandlerImpl) {
        let pool = web::Data::new(fixtures_db::create_undefined_db().await);
        let (dao, handler) = get_dao_and_handler();
        (pool, dao, handler)
    }

    fn get_dao_and_handler() -> (web::Data<AccountDaoImpl>, AccountHandlerImpl) {
        let dao = web::Data::new(AccountDaoImpl);
        let handler = AccountHandlerImpl;
        (dao, handler)
    }

    mod create_account {
        use super::*;

        #[actix_web::test]
        async fn create_success() {
            // preparation
            let (pool, dao, handler) = setup_empty().await;
            let new_account = fixtures_accounts::get_first_account();
            let new_account_json = web::Json(new_account.clone());
            // execution
            let resp = handler.create_account(pool, dao, new_account_json).await.unwrap();
            // assersion
            assert_eq!(resp.status(), StatusCode::CREATED);
            let body_bytes = to_bytes(resp.into_body()).await.unwrap();
            let body: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
            assert_eq!(body, json!({"created": 1}));
        }

        #[actix_web::test]
        async fn create_conflict() {
            // preparation
            let (pool, dao, handler) = setup().await;
            let existing_account = fixtures_accounts::get_first_account();
            let existing_account_json = web::Json(existing_account.clone());
            // execution
            let resp = handler.create_account(pool, dao, existing_account_json).await.unwrap();
            // assersion
            assert_eq!(resp.status(), StatusCode::CONFLICT);
            let body_bytes = to_bytes(resp.into_body()).await.unwrap();
            let body: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
            assert_eq!(body, json!({"error": "Account ID already exists."}));
        }

        #[actix_web::test]
        async fn create_servererror_on_check_exists() {
            // preparation
            let (pool, dao, handler) = setup_undefined().await;
            let new_account = fixtures_accounts::get_first_account();
            let new_account_json = web::Json(new_account.clone());
            // execution
            let resp = handler.create_account(pool, dao, new_account_json).await.unwrap();
            // assersion
            assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);
            let body_bytes = to_bytes(resp.into_body()).await.unwrap();
            let body: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
            assert_eq!(body, json!({"error": "Failed to fetch account."}));
        }

        // TODO: モックを使えるようになってから
        // #[actix_web::test]
        // async fn create_servererror_on_creation() {
        //     // preparation
        //     let (pool, dao, handler) = setup_undefined().await;
        //     let new_account = fixtures_accounts::get_first_account();
        //     let new_account_json = web::Json(new_account.clone());
        //     // execution
        //     let resp = handler.create_account(pool, dao, new_account_json).await.unwrap();
        //     // assersion
        //     assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);
        //     let body_bytes = to_bytes(resp.into_body()).await.unwrap();
        //     let body: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
        //     assert_eq!(body, json!({"error": "Failed to create account."}));
        // }
    }

    mod get_accounts_list_all {
        use super::*;

        #[actix_web::test]
        async fn get_all_is_empty() {
            // preparation
            let (pool, dao, handler) = setup_empty().await;
            // execution
            let resp = handler.get_accounts_list_all(pool, dao).await.unwrap();
            // assersion
            assert_eq!(resp.status(), StatusCode::OK);
            let body_bytes = to_bytes(resp.into_body()).await.unwrap();
            let accounts: Vec<Account> = serde_json::from_slice(&body_bytes).unwrap();
            assert_eq!(accounts.len(), 0);
        }

        #[actix_web::test]
        async fn get_all_found_3accounts() {
            // preparation
            let (pool, dao, handler) = setup().await;
            let account_list = fixtures_accounts::get_sorted_account_list();
            // execution
            let resp = handler.get_accounts_list_all(pool, dao).await.unwrap();
            // assersion
            assert_eq!(resp.status(), StatusCode::OK);
            let body_bytes = to_bytes(resp.into_body()).await.unwrap();
            let accounts: Vec<Account> = serde_json::from_slice(&body_bytes).unwrap();
            assert_eq!(accounts.len(), 3);
            assert_eq!(accounts[0].id, account_list[0].id);
            assert_eq!(accounts[1].id, account_list[1].id);
            assert_eq!(accounts[2].id, account_list[2].id);
        }

        #[actix_web::test]
        async fn get_all_servererror() {
            // preparation
            let (pool, dao, handler) = setup_undefined().await;
            // execution
            let resp = handler.get_accounts_list_all(pool, dao).await.unwrap();
            // assersion
            assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);
            let body_bytes = to_bytes(resp.into_body()).await.unwrap();
            let body: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
            assert_eq!(body, json!({"error": "Failed to fetch accounts."}));
        }
    }

    mod get_accounts_list_by_type {
        use super::*;

        #[actix_web::test]
        async fn get_type_not_found() {
            // preparation
            let (pool, dao, handler) = setup().await;
            let path = web::Path::from("Other".to_string());
            // execution
            let resp = handler.get_accounts_list_by_type(pool, dao, path).await.unwrap();
            // assersion
            assert_eq!(resp.status(), StatusCode::NOT_FOUND);
            let body_bytes = to_bytes(resp.into_body()).await.unwrap();
            let body: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
            assert_eq!(body, json!({"error": "No accounts found."}));
        }

        #[actix_web::test]
        async fn get_type_found() {
            // preparation
            let (pool, dao, handler) = setup().await;
            let account = fixtures_accounts::get_first_account();
            let path = web::Path::from(account.account_type.name.clone());
            // execution
            let resp = handler.get_accounts_list_by_type(pool, dao, path).await.unwrap();
            // assersion
            assert_eq!(resp.status(), StatusCode::OK);
            let body_bytes = to_bytes(resp.into_body()).await.unwrap();
            let accounts: Vec<Account> = serde_json::from_slice(&body_bytes).unwrap();
            assert_eq!(accounts.len(), 1);
            assert_eq!(accounts[0].id, account.id);
        }

        #[actix_web::test]
        async fn get_type_servererror() {
            // preparation
            let (pool, dao, handler) = setup_undefined().await;
            let account = fixtures_accounts::get_first_account();
            let path = web::Path::from(account.account_type.name.clone());
            // execution
            let resp = handler.get_accounts_list_by_type(pool, dao, path).await.unwrap();
            // assersion
            assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);
            let body_bytes = to_bytes(resp.into_body()).await.unwrap();
            let body: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
            assert_eq!(body, json!({"error": "Failed to fetch accounts."}));
        }
    }

    mod get_accounts_by_id {
        use super::*;

        #[actix_web::test]
        async fn get_by_id_not_found() {
            // preparation
            let (pool, dao, handler) = setup().await;
            let path = web::Path::from("nonexistent_id".to_string());
            // execution
            let resp = handler.get_account_by_id(pool, dao, path).await.unwrap();
            // assersion
            assert_eq!(resp.status(), StatusCode::NOT_FOUND);
            let body_bytes = to_bytes(resp.into_body()).await.unwrap();
            let body: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
            assert_eq!(body, json!({"error": "Account not found."}));
        }

        #[actix_web::test]
        async fn get_by_id_found() {
            // preparation
            let (pool, dao, handler) = setup().await;
            let account = fixtures_accounts::get_first_account();
            let path = web::Path::from(account.id.clone());
            // execution
            let resp = handler.get_account_by_id(pool, dao, path).await.unwrap();
            // assersion
            assert_eq!(resp.status(), StatusCode::OK);
            let body_bytes = to_bytes(resp.into_body()).await.unwrap();
            let fetched_account: Account = serde_json::from_slice(&body_bytes).unwrap();
            assert_eq!(fetched_account.id, account.id);
        }

        #[actix_web::test]
        async fn get_by_id_servererror() {
            // preparation
            let (pool, dao, handler) = setup_undefined().await;
            let path = web::Path::from("any_id".to_string());
            // execution
            let resp = handler.get_account_by_id(pool, dao, path).await.unwrap();
            // assersion
            assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);
            let body_bytes = to_bytes(resp.into_body()).await.unwrap();
            let body: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
            assert_eq!(body, json!({"error": "Failed to fetch account."}));
        }
    }

    mod delete_account {
        use super::*;

        #[actix_web::test]
        async fn delete_not_found() {
            // preparation
            let (pool, dao, handler) = setup().await;
            let path = web::Path::from("nonexistent_id".to_string());
            // execution
            let resp = handler.delete_account(pool, dao, path).await.unwrap();
            // assersion
            assert_eq!(resp.status(), StatusCode::NOT_FOUND);
            let body_bytes = to_bytes(resp.into_body()).await.unwrap();
            let body: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
            assert_eq!(body, json!({"error": "Account not found."}));
        }

        #[actix_web::test]
        async fn delete_success() {
            // preparation
            let (pool, dao, handler) = setup().await;
            let account = fixtures_accounts::get_first_account();
            let path = web::Path::from(account.id.clone());
            // execution
            let resp = handler.delete_account(pool, dao, path).await.unwrap();
            // assersion
            assert_eq!(resp.status(), StatusCode::NO_CONTENT);
            let body_bytes = to_bytes(resp.into_body()).await.unwrap();
            let body: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
            assert_eq!(body, json!({"message": "Account deleted successfully."}));
        }

        #[actix_web::test]
        async fn delete_servererror() {
            // preparation
            let (pool, dao, handler) = setup_undefined().await;
            let path = web::Path::from("any_id".to_string());
            // execution
            let resp = handler.delete_account(pool, dao, path).await.unwrap();
            // assersion
            assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);
            let body_bytes = to_bytes(resp.into_body()).await.unwrap();
            let body: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
            assert_eq!(body, json!({"error": "Failed to delete account."}));
        }
    }

    mod update_account {
        use super::*;

        #[actix_web::test]
        async fn update_not_found() {
            // preparation
            let (pool, dao, handler) = setup().await;
            let account = fixtures_accounts::create_new_account();
            let account_json = web::Json(account);
            // execution
            let resp = handler.update_account(pool, dao, account_json).await.unwrap();
            // assersion
            assert_eq!(resp.status(), StatusCode::NOT_FOUND);
            let body_bytes = to_bytes(resp.into_body()).await.unwrap();
            let body: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
            assert_eq!(body, json!({"error": "Account not found."}));
        }

        #[actix_web::test]
        async fn update_success() {
            // preparation
            let (pool, dao, handler) = setup().await;
            let mut account = fixtures_accounts::get_first_account();
            account.memo = Some("更新されたメモ".to_string());
            let account_json = web::Json(account.clone());
            // execution
            let resp = handler.update_account(pool, dao, account_json).await.unwrap();
            // assersion
            assert_eq!(resp.status(), StatusCode::OK);
            let body_bytes = to_bytes(resp.into_body()).await.unwrap();
            let body: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
            assert_eq!(body, json!({"updated": 1}));
        }

        #[actix_web::test]
        async fn update_servererror() {
            // preparation
            let (pool, dao, handler) = setup_undefined().await;
            let account = fixtures_accounts::get_first_account();
            let account_json = web::Json(account);
            // execution
            let resp = handler.update_account(pool, dao, account_json).await.unwrap();
            // assersion
            assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);
            let body_bytes = to_bytes(resp.into_body()).await.unwrap();
            let body: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
            assert_eq!(body, json!({"error": "Failed to update account."}));
        }
    }
}
