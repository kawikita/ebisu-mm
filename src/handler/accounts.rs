use crate::dao::accounts::AccountDao;
use crate::entity::accounts::Account;
use crate::handler::HandlerResult;
use crate::utils::app_setup::AppModule;
use crate::utils::error::{AppError, ErrorResponse};
use crate::utils::message::{MessageHierarchy, set_hierarchy};
use actix_web::{HttpResponse, web};
use log::info;
use once_cell::sync::Lazy;
use serde_json::json;
use shaku::{Component, HasComponent, Interface};
use sqlx::SqlitePool;
use std::sync::Arc;

const MODULE_PATH: &str = module_path!();
const API_BASE_PATH: &str = "/api/account";
static MESSAGE: Lazy<MessageHierarchy<'static>> = Lazy::new(|| set_hierarchy(MODULE_PATH));

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
                    .route(web::get().to(get_accounts_list_all_handler))
                    .route(web::post().to(create_account_handler))
                    .route(web::put().to(update_account_handler)),
            )
            .service(
                web::resource("/{id}")
                    .route(web::get().to(get_account_by_id_handler))
                    .route(web::delete().to(delete_account_handler)),
            )
            .service(web::resource("/type/{type_name}").route(web::get().to(get_accounts_list_by_type_handler))),
    );
}

// utoipaのpath登録用のラッパー関数

#[utoipa::path(post, path = "/api/account", tag = "accounts", request_body = Account, responses(
    (status = 201, description = "Account created successfully", body = serde_json::Value, example = json!({"created": 1})),
    (status = 409, description = "Account ID already exists", body = ErrorResponse, example = json!(AppError::Conflict(MESSAGE.get("account_id_already_exists")).to_error_response())),
    (status = 500, description = "Internal server error", body = ErrorResponse, example = json!(AppError::InternalServerError(MESSAGE.get("failed_to_create_account")).to_error_response())),
))]
/// 新しい口座情報を作成するAPI
/// # Arguments
/// * `pool` - Sqliteのコネクションプール
/// * `app_module` - アプリケーションのDIコンテナ
/// * `data` - リクエストボディから取得した口座情報
pub async fn create_account_handler(pool: web::Data<SqlitePool>, app_module: web::Data<AppModule>, data: web::Json<Account>) -> HandlerResult {
    let handler: Arc<dyn AccountHandler> = app_module.resolve();
    handler.create_account(pool, data).await
}

#[utoipa::path(delete, path = "/api/account/{id}", tag = "accounts", params(
    ("id" = String, Path, description = "Account ID(UUID)")
), responses(
    (status = 204, description = "Account deleted successfully"),
    (status = 404, description = "Account not found", body = ErrorResponse, example = json!(AppError::NotFound(MESSAGE.get("account_not_found")).to_error_response())),
    (status = 500, description = "Internal server error", body = ErrorResponse, example = json!(AppError::InternalServerError(MESSAGE.get("failed_to_delete_account")).to_error_response())),
))]
/// 指定されたIDの口座情報を削除するAPI
/// # Arguments
/// * `pool` - Sqliteのコネクションプール
/// * `app_module` - アプリケーションのDIコンテナ
/// * `path` - リクエストパスから取得した口座ID
/// # Returns
/// 成功時はHTTP 204、失敗時はHTTP 500とエラーメッセージを返す
pub async fn delete_account_handler(pool: web::Data<SqlitePool>, app_module: web::Data<AppModule>, path: web::Path<String>) -> HandlerResult {
    let handler: Arc<dyn AccountHandler> = app_module.resolve();
    handler.delete_account(pool, path).await
}

#[utoipa::path(get, path = "/api/account/{id}", tag = "accounts", params(
    ("id" = String, Path, description = "Account ID(UUID)")
), responses(
    (status = 200, description = "Account fetched successfully", body = Account),
    (status = 404, description = "Account not found", body = ErrorResponse, example = json!(AppError::NotFound(MESSAGE.get("account_not_found")).to_error_response())),
    (status = 500, description = "Internal server error", body = ErrorResponse, example = json!(AppError::InternalServerError(MESSAGE.get("failed_to_fetch_account")).to_error_response())),
))]
/// 指定されたIDの口座情報を取得するAPI
/// # Arguments
/// * `pool` - Sqliteのコネクションプール
/// * `app_module` - アプリケーションのDIコンテナ
/// * `path` - リクエストパスから取得した口座ID
/// # Returns
/// 成功時はHTTP 200、失敗時はHTTP 500とエラーメッセージを返す
pub async fn get_account_by_id_handler(pool: web::Data<SqlitePool>, app_module: web::Data<AppModule>, path: web::Path<String>) -> HandlerResult {
    let handler: Arc<dyn AccountHandler> = app_module.resolve();
    handler.get_account_by_id(pool, path).await
}

#[utoipa::path(get, path = "/api/account", tag = "accounts", responses(
    (status = 200, description = "Accounts fetched successfully", body = [Account]),
    (status = 404, description = "Account not found", body = ErrorResponse, example = json!(AppError::NotFound(MESSAGE.get("account_not_found")).to_error_response())),
    (status = 500, description = "Internal server error", body = ErrorResponse, example = json!(AppError::InternalServerError(MESSAGE.get("failed_to_fetch_accounts")).to_error_response()))
))]
/// 全ての口座情報を取得するAPI
/// # Arguments
/// * `pool` - Sqliteのコネクションプール
/// * `app_module` - アプリケーションのDIコンテナ
/// # Returns
/// 成功時はHTTP 200、失敗時はHTTP 500とエラーメッセージを返す
pub async fn get_accounts_list_all_handler(pool: web::Data<SqlitePool>, app_module: web::Data<AppModule>) -> HandlerResult {
    let handler: Arc<dyn AccountHandler> = app_module.resolve();
    handler.get_accounts_list_all(pool).await
}

#[utoipa::path(get, path = "/api/account/type/{type_name}", tag = "accounts", params(
    ("type_name" = String, Path, description = "Account Type Name")
), responses(
    (status = 200, description = "Accounts fetched successfully", body = [Account]),
    (status = 404, description = "No accounts found", body = ErrorResponse, example = json!(AppError::NotFound(MESSAGE.get("no_accounts_found")).to_error_response())),
    (status = 500, description = "Internal server error", body = ErrorResponse, example = json!(AppError::InternalServerError(MESSAGE.get("failed_to_fetch_accounts")).to_error_response())),
))]
/// 指定された口座種別の口座情報を取得するAPI
/// # Arguments
/// * `pool` - Sqliteのコネクションプール
/// * `app_module` - アプリケーションのDIコンテナ
/// * `path` - リクエストパスから取得した口座種別名
/// # Returns
/// 成功時はHTTP 200、失敗時はHTTP 500とエラーメッセージを返す
pub async fn get_accounts_list_by_type_handler(pool: web::Data<SqlitePool>, app_module: web::Data<AppModule>, path: web::Path<String>) -> HandlerResult {
    let handler: Arc<dyn AccountHandler> = app_module.resolve();
    handler.get_accounts_list_by_type(pool, path).await
}

#[utoipa::path(put, path = "/api/account", tag = "accounts", request_body = Account, responses(
    (status = 200, description = "Account updated successfully", body = serde_json::Value, example = json!({"updated": 1})),
    (status = 404, description = "Account not found", body = ErrorResponse, example = json!(AppError::NotFound(MESSAGE.get("account_not_found")).to_error_response())),
    (status = 500, description = "Internal server error", body = ErrorResponse, example = json!(AppError::InternalServerError(MESSAGE.get("failed_to_update_account")).to_error_response())),
))]
/// 指定されたIDの口座情報を更新するAPI
/// # Arguments
/// * `pool` - Sqliteのコネクションプール
/// * `app_module` - アプリケーションのDIコンテナ
/// * `data` - リクエストボディから取得した口座情報
/// # Returns
/// 成功時はHTTP 200、失敗時はHTTP 500とエラーメッセージを返す
pub async fn update_account_handler(pool: web::Data<SqlitePool>, app_module: web::Data<AppModule>, data: web::Json<Account>) -> HandlerResult {
    let handler: Arc<dyn AccountHandler> = app_module.resolve();
    handler.update_account(pool, data).await
}

/// 口座情報ハンドラーのインターフェース
#[async_trait::async_trait]
pub trait AccountHandler: Interface {
    async fn create_account(&self, pool: web::Data<SqlitePool>, data: web::Json<Account>) -> HandlerResult;
    async fn delete_account(&self, pool: web::Data<SqlitePool>, path: web::Path<String>) -> HandlerResult;
    async fn get_account_by_id(&self, pool: web::Data<SqlitePool>, path: web::Path<String>) -> HandlerResult;
    async fn get_accounts_list_all(&self, pool: web::Data<SqlitePool>) -> HandlerResult;
    async fn get_accounts_list_by_type(&self, pool: web::Data<SqlitePool>, path: web::Path<String>) -> HandlerResult;
    async fn update_account(&self, pool: web::Data<SqlitePool>, data: web::Json<Account>) -> HandlerResult;
}

/// 口座情報ハンドラーの実装
/// # Fields
/// * `account_dao` - 口座情報DAOのインスタンス
#[derive(Clone, Component)]
#[shaku(interface = AccountHandler)]
pub struct AccountHandlerImpl {
    #[shaku(inject)]
    account_dao: Arc<dyn AccountDao>,
}

#[async_trait::async_trait]
impl AccountHandler for AccountHandlerImpl {
    /// 新しい口座情報を作成するハンドラー関数
    /// # Arguments
    /// * `pool` - データベース接続プール
    /// * `data` - リクエストボディから取得した新しい口座情報
    /// # Returns
    /// 成功時はHTTP 201と作成件数、失敗時はHTTP 500とエラーメッセージ
    async fn create_account(&self, pool: web::Data<SqlitePool>, data: web::Json<Account>) -> HandlerResult {
        info!("Received request to create a new account");
        let account_id = data.id.clone();
        // IDの重複チェック
        let account_body =
            self.account_dao
                .get_account_by_id(&pool, &account_id)
                .await
                .map_err(|e| {
                    log::error!(
                        "Failed to fetch account by ID {}: {:?}",
                        account_id,
                        e
                    );
                    AppError::InternalServerError(MESSAGE.get("failed_to_fetch_account"))
                })?;
        if account_body.is_some() {
            info!("Account with ID {} already exists.", account_id);
            return Err(AppError::Conflict(MESSAGE.get("account_id_already_exists")));
        }
        let success_count =
            self.account_dao
                .create_account(&pool, &data)
                .await
                .map_err(|e| {
                    log::error!(
                        "Failed to create account with ID {}: {:?}",
                        account_id,
                        e
                    );
                    AppError::InternalServerError(MESSAGE.get("failed_to_create_account"))
                })?;
        info!("Created {} new account(s).", success_count);
        Ok(HttpResponse::Created().json(json!({"created": success_count})))
    }

    /// 指定されたIDの口座情報を削除するハンドラー関数
    /// # Arguments
    /// * `pool` - データベース接続プール
    /// * `path` - URLパスから取得した口座ID
    /// # Returns
    /// 成功時はHTTP 200と成功メッセージ、失敗時はHTTP 500とエラーメッセージ
    async fn delete_account(&self, pool: web::Data<SqlitePool>, path: web::Path<String>) -> HandlerResult {
        let account_id = path.as_str();
        info!("Received request to delete account by ID: {}", account_id);
        let del_count =
            self.account_dao.delete_account(&pool, account_id).await.map_err(|_| AppError::InternalServerError(MESSAGE.get("failed_to_delete_account")))?;
        if del_count == 0 {
            info!("No account found with ID: {}", account_id);
            return Err(AppError::NotFound(MESSAGE.get("account_not_found")));
        }
        info!("Deleted account with ID: {}", account_id);
        Ok(HttpResponse::NoContent().finish())
    }

    /// 指定されたIDの口座情報を取得するハンドラー関数
    /// # Arguments
    /// * `pool` - データベース接続プール
    /// * `path` - URLパスから取得した口座ID
    /// # Returns
    /// 成功時はHTTP 200と口座情報のJSON、失敗時はHTTP 500とエラーメッセージ
    async fn get_account_by_id(&self, pool: web::Data<SqlitePool>, path: web::Path<String>) -> HandlerResult {
        let account_id = path.as_str();
        info!("Received request to fetch account by ID: {}", account_id);
        let account_body =
            self.account_dao.get_account_by_id(&pool, account_id).await.map_err(|_| AppError::InternalServerError(MESSAGE.get("failed_to_fetch_account")))?;
        match account_body {
            Some(account) => {
                info!("Fetched account with ID: {}", account_id);
                Ok(HttpResponse::Ok().json(account))
            }
            None => {
                info!("No account found with ID: {}", account_id);
                Err(AppError::NotFound(MESSAGE.get("account_not_found")))
            }
        }
    }

    /// 全ての口座情報を取得するハンドラー関数
    /// # Arguments
    /// * `pool` - データベース接続プール
    /// # Returns
    /// 成功時はHTTP 200と口座情報のJSON配列、失敗時はHTTP 500とエラーメッセージ
    /// 全ての口座情報が1件も存在しない場合はHTTP 404とエラーメッセージを返す
    async fn get_accounts_list_all(&self, pool: web::Data<SqlitePool>) -> HandlerResult {
        info!("Received request to fetch all accounts");
        let account_list =
            self.account_dao.get_accounts_list_all(&pool).await.map_err(|_| AppError::InternalServerError(MESSAGE.get("failed_to_fetch_accounts")))?;
        if account_list.is_empty() {
            return Err(AppError::NotFound(MESSAGE.get("no_accounts_found")));
        }
        info!("Fetched {} accounts.", account_list.len());
        Ok(HttpResponse::Ok().json(account_list))
    }

    /// 指定された口座種別の口座情報を取得するハンドラー関数
    /// # Arguments
    /// * `pool` - データベース接続プール
    /// * `type_name` - URLパスから取得した口座種別名
    /// # Returns
    /// 成功時はHTTP 200と口座情報のJSON配列、失敗時はHTTP 500とエラーメッセージ
    /// 指定された口座種別の口座情報が1件も存在しない場合はHTTP 404とエラーメッセージを返す
    async fn get_accounts_list_by_type(&self, pool: web::Data<SqlitePool>, type_name: web::Path<String>) -> HandlerResult {
        info!("Received request to fetch accounts of type: {}", type_name);
        let account_list = self
            .account_dao
            .get_accounts_list_by_type(&pool, &type_name)
            .await
            .map_err(|_| AppError::InternalServerError(MESSAGE.get("failed_to_fetch_accounts")))?;
        if account_list.is_empty() {
            return Err(AppError::NotFound(MESSAGE.get("no_accounts_found")));
        }
        info!("Fetched {} accounts of type: {}", account_list.len(), type_name);
        Ok(HttpResponse::Ok().json(account_list))
    }

    /// 指定されたIDの口座情報を更新するハンドラー関数
    /// # 引数
    /// * `pool` - データベース接続プール
    /// * `dao` - 口座情報DAO
    /// * `data` - リクエストボディから取得した更新後の口座情報
    /// # 戻り値
    /// 成功時はHTTP 200と更新された口座情報のJSON、失敗時はHTTP 500とエラーメッセージ
    async fn update_account(&self, pool: web::Data<SqlitePool>, data: web::Json<Account>) -> HandlerResult {
        info!("Received request to update account by ID: {}", data.id);
        let update = self.account_dao.update_account(&pool, &data).await.map_err(|_| AppError::InternalServerError(MESSAGE.get("failed_to_update_account")))?;

        if update == 0 {
            return Err(AppError::NotFound(MESSAGE.get("account_not_found")));
        }
        info!("Updated account with ID: {}", data.id);
        Ok(HttpResponse::Ok().json(json!({"updated": update})))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::unit_test::fixtures::accounts as fixtures_accounts;
    use crate::utils::unit_test::fixtures::accounts::{MockConfig, ParametrizedMockAccountDaoImpl, ParametrizedMockAccountDaoImplParameters};
    use crate::utils::unit_test::fixtures::app_test_setup::TestAppModule;
    use crate::utils::unit_test::fixtures::db as fixtures_db;
    use actix_web::body::to_bytes;
    use actix_web::http::StatusCode;

    async fn setup(parameters: MockConfig) -> (web::Data<SqlitePool>, Arc<dyn AccountHandler>) {
        let pool = web::Data::new(fixtures_db::create_undefined_db().await);
        let add_module = TestAppModule::builder()
            .with_component_parameters::<ParametrizedMockAccountDaoImpl>(ParametrizedMockAccountDaoImplParameters { config: Arc::new(parameters) })
            .build();
        (pool, add_module.resolve())
    }

    pub fn get_normal_params() -> MockConfig {
        MockConfig {
            error_on_create: false,
            error_on_get: false,
            error_on_update: false,
            error_on_delete: false,
            get_return_empty: false,
            get_in_create_return_empty: true,
            create_return_empty: false,
            update_return_empty: false,
            delete_return_empty: false,
        }
    }

    pub fn get_exists_account_params() -> MockConfig {
        MockConfig {
            error_on_create: false,
            error_on_get: false,
            error_on_update: false,
            error_on_delete: false,
            get_return_empty: false,
            get_in_create_return_empty: false,
            create_return_empty: false,
            update_return_empty: false,
            delete_return_empty: false,
        }
    }

    pub fn get_empty_params() -> MockConfig {
        MockConfig {
            error_on_create: false,
            error_on_get: false,
            error_on_update: false,
            error_on_delete: false,
            get_return_empty: true,
            get_in_create_return_empty: true,
            create_return_empty: true,
            update_return_empty: true,
            delete_return_empty: true,
        }
    }

    pub fn get_error_params() -> MockConfig {
        MockConfig {
            error_on_create: true,
            error_on_get: true,
            error_on_update: true,
            error_on_delete: true,
            get_return_empty: false,
            get_in_create_return_empty: true,
            create_return_empty: false,
            update_return_empty: false,
            delete_return_empty: false,
        }
    }

    pub fn get_error_on_creation_params() -> MockConfig {
        MockConfig {
            error_on_create: true,
            error_on_get: false,
            error_on_update: true,
            error_on_delete: true,
            get_return_empty: false,
            get_in_create_return_empty: true,
            create_return_empty: false,
            update_return_empty: false,
            delete_return_empty: false,
        }
    }

    mod create_account {

        use super::*;

        #[tokio::test]
        async fn create_success() {
            // preparation
            let (pool, _handler) = setup(get_normal_params()).await;
            let new_account = fixtures_accounts::get_first_account();
            let new_account_json = web::Json(new_account.clone());
            // execution
            let resp = _handler.create_account(pool, new_account_json).await.unwrap();
            // assertion
            assert_eq!(resp.status(), StatusCode::CREATED);
            let body_bytes = to_bytes(resp.into_body()).await.unwrap();
            let body: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
            assert_eq!(body, json!({"created": 1}));
        }

        #[tokio::test]
        async fn create_conflict() {
            // preparation
            let (pool, _handler) = setup(get_exists_account_params()).await;
            let existing_account = fixtures_accounts::get_first_account();
            let existing_account_json = web::Json(existing_account.clone());
            // execution
            let resp = _handler.create_account(pool, existing_account_json).await.unwrap_err();
            // assertion
            match resp {
                AppError::Conflict(message) => {
                    assert_eq!(message, "Account ID already exists.");
                },
                _ => panic!("Expected Conflict error"),
            }
        }

        #[tokio::test]
        async fn create_servererror_on_check_exists() {
            // preparation
            let (pool, _handler) = setup(get_error_params()).await;
            let new_account = fixtures_accounts::get_first_account();
            let new_account_json = web::Json(new_account.clone());
            // execution
            let resp = _handler.create_account(pool, new_account_json).await.unwrap_err();
            // assertion
            match resp {
                AppError::InternalServerError(message) => {
                    assert_eq!(message, "Failed to fetch account.");
                },
                _ => panic!("Expected InternalServerError error"),
            }
        }

        // TODO: モックを使えるようになってから
        #[tokio::test]
        async fn create_servererror_on_creation() {
            // preparation
            let (pool, _handler) = setup(get_error_on_creation_params()).await;
            let new_account = fixtures_accounts::get_first_account();
            let new_account_json = web::Json(new_account.clone());
            // execution
            let resp = _handler.create_account(pool, new_account_json).await.unwrap_err();
            // assertion
            match resp {
                AppError::InternalServerError(message) => {
                    assert_eq!(message, "Failed to create account.");
                },
                _ => panic!("Expected InternalServerError error"),
            }
        }
    }

    mod get_accounts_list_all {
        use super::*;

        #[tokio::test]
        async fn get_all_is_empty() {
            // preparation
            let (pool, _handler) = setup(get_empty_params()).await;
            // execution
            let resp = _handler.get_accounts_list_all(pool).await.unwrap_err();
            // assertion
            match resp {
                AppError::NotFound(message) => {
                    assert_eq!(message, "No accounts found.");
                },
                _ => panic!("Expected NotFound error"),
            }
        }

        #[tokio::test]
        async fn get_all_found_3accounts() {
            // preparation
            let (pool, _handler) = setup(get_normal_params()).await;
            let account_list = fixtures_accounts::get_sorted_account_list();
            // execution
            let resp = _handler.get_accounts_list_all(pool).await.unwrap();
            // assertion
            assert_eq!(resp.status(), StatusCode::OK);
            let body_bytes = to_bytes(resp.into_body()).await.unwrap();
            let accounts: Vec<Account> = serde_json::from_slice(&body_bytes).unwrap();
            assert_eq!(accounts.len(), 3);
            assert_eq!(accounts[0].id, account_list[0].id);
            assert_eq!(accounts[1].id, account_list[1].id);
            assert_eq!(accounts[2].id, account_list[2].id);
        }

        #[tokio::test]
        async fn get_all_servererror() {
            // preparation
            let (pool, _handler) = setup(get_error_params()).await;
            // execution
            let resp = _handler.get_accounts_list_all(pool).await.unwrap_err();
            // assertion
            match resp {
                AppError::InternalServerError(message) => {
                    assert_eq!(message, "Failed to fetch accounts.");
                },
                _ => panic!("Expected InternalServerError error"),
            }
        }
    }

    mod get_accounts_list_by_type {
        use super::*;

        #[tokio::test]
        async fn get_type_not_found() {
            // preparation
            let (pool, _handler) = setup(get_empty_params()).await;
            let path = web::Path::from("Other".to_string());
            // execution
            let resp = _handler.get_accounts_list_by_type(pool, path).await.unwrap_err();
            // assertion
            match resp {
                AppError::NotFound(message) => {
                    assert_eq!(message, "No accounts found.");
                },
                _ => panic!("Expected NotFound error"),
            }
        }

        #[tokio::test]
        async fn get_type_found() {
            // preparation
            let (pool, _handler) = setup(get_normal_params()).await;
            let account = fixtures_accounts::get_first_account();
            let path = web::Path::from(account.account_type.name.clone());
            // execution
            let resp = _handler.get_accounts_list_by_type(pool, path).await.unwrap();
            // assertion
            assert_eq!(resp.status(), StatusCode::OK);
            let body_bytes = to_bytes(resp.into_body()).await.unwrap();
            let accounts: Vec<Account> = serde_json::from_slice(&body_bytes).unwrap();
            assert_eq!(accounts.len(), 1);
            assert_eq!(accounts[0].id, account.id);
        }

        #[tokio::test]
        async fn get_type_servererror() {
            // preparation
            let (pool, _handler) = setup(get_error_params()).await;
            let account = fixtures_accounts::get_first_account();
            let path = web::Path::from(account.account_type.name.clone());
            // execution
            let resp = _handler.get_accounts_list_by_type(pool, path).await.unwrap_err();
            // assertion
            match resp {
                AppError::InternalServerError(message) => {
                    assert_eq!(message, "Failed to fetch accounts.");
                },
                _ => panic!("Expected InternalServerError error"),
            }
        }
    }

    mod get_accounts_by_id {
        use super::*;

        #[tokio::test]
        async fn get_by_id_not_found() {
            // preparation
            let (pool, _handler) = setup(get_empty_params()).await;
            let path = web::Path::from("nonexistent_id".to_string());
            // execution
            let resp = _handler.get_account_by_id(pool, path).await.unwrap_err();
            // assertion
            match resp {
                AppError::NotFound(message) => {
                    assert_eq!(message, "Account not found.");
                },
                _ => panic!("Expected NotFound error"),
            }
        }

        #[tokio::test]
        async fn get_by_id_found() {
            // preparation
            let (pool, _handler) = setup(get_normal_params()).await;
            let account = fixtures_accounts::get_first_account();
            let path = web::Path::from(account.id.clone());
            // execution
            let resp = _handler.get_account_by_id(pool, path).await.unwrap();
            // assertion
            assert_eq!(resp.status(), StatusCode::OK);
            let body_bytes = to_bytes(resp.into_body()).await.unwrap();
            let fetched_account: Account = serde_json::from_slice(&body_bytes).unwrap();
            assert_eq!(fetched_account.id, account.id);
        }

        #[tokio::test]
        async fn get_by_id_servererror() {
            // preparation
            let (pool, _handler) = setup(get_error_params()).await;
            let path = web::Path::from("any_id".to_string());
            // execution
            let resp = _handler.get_account_by_id(pool, path).await.unwrap_err();
            // assertion
            match resp {
                AppError::InternalServerError(message) => {
                    assert_eq!(message, "Failed to fetch account.");
                },
                _ => panic!("Expected InternalServerError error"),
            }
        }
    }

    mod delete_account {
        use super::*;

        #[tokio::test]
        async fn delete_not_found() {
            // preparation
            let (pool, _handler) = setup(get_empty_params()).await;
            let path = web::Path::from("nonexistent_id".to_string());
            // execution
            let resp = _handler.delete_account(pool, path).await.unwrap_err();
            // assertion
            match resp {
                AppError::NotFound(message) => {
                    assert_eq!(message, "Account not found.");
                },
                _ => panic!("Expected NotFound error"),
            }
        }

        #[tokio::test]
        async fn delete_success() {
            // preparation
            let (pool, _handler) = setup(get_normal_params()).await;
            let account = fixtures_accounts::get_first_account();
            let path = web::Path::from(account.id.clone());
            // execution
            let resp = _handler.delete_account(pool, path).await.unwrap();
            // assertion
            assert_eq!(resp.status(), StatusCode::NO_CONTENT);
        }

        #[tokio::test]
        async fn delete_servererror() {
            // preparation
            let (pool, _handler) = setup(get_error_params()).await;
            let path = web::Path::from("any_id".to_string());
            // execution
            let resp = _handler.delete_account(pool, path).await.unwrap_err();
            // assertion
            match resp {
                AppError::InternalServerError(message) => {
                    assert_eq!(message, "Failed to delete account.");
                },
                _ => panic!("Expected InternalServerError error"),
            }
        }
    }

    mod update_account {
        use super::*;

        #[tokio::test]
        async fn update_not_found() {
            // preparation
            let (pool, _handler) = setup(get_empty_params()).await;
            let account = fixtures_accounts::create_new_account();
            let account_json = web::Json(account);
            // execution
            let resp = _handler.update_account(pool, account_json).await.unwrap_err();
            // assertion
            match resp {
                AppError::NotFound(message) => {
                    assert_eq!(message, "Account not found.");
                },
                _ => panic!("Expected NotFound error"),
            }
        }

        #[tokio::test]
        async fn update_success() {
            // preparation
            let (pool, _handler) = setup(get_normal_params()).await;
            let mut account = fixtures_accounts::get_first_account();
            account.memo = Some("更新されたメモ".to_string());
            let account_json = web::Json(account.clone());
            // execution
            let resp = _handler.update_account(pool, account_json).await.unwrap();
            // assertion
            assert_eq!(resp.status(), StatusCode::OK);
            let body_bytes = to_bytes(resp.into_body()).await.unwrap();
            let body: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
            assert_eq!(body, json!({"updated": 1}));
        }

        #[tokio::test]
        async fn update_servererror() {
            // preparation
            let (pool, _handler) = setup(get_error_params()).await;
            let account = fixtures_accounts::get_first_account();
            let account_json = web::Json(account);
            // execution
            let resp = _handler.update_account(pool, account_json).await.unwrap_err();
            // assertion
            match resp {
                AppError::InternalServerError(message) => {
                    assert_eq!(message, "Failed to update account.");
                },
                _ => panic!("Expected InternalServerError error"),
            }
        }
    }
}
