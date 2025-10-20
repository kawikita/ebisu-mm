use crate::dao::accounts::AccountDao;
use crate::entity::accounts::Account;
use crate::utils::app_setup::AppModule;
use crate::utils::message::{MessageHierarchy, set_hierarchy};
use actix_web::{HttpResponse, Result, web};
use log::{error, info};
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
    (status = 409, description = "Account ID already exists", body = serde_json::Value, example = json!({"error": MESSAGE.get("account_id_already_exists")})),
    (status = 500, description = "Internal server error", body = serde_json::Value, example = json!({"error": MESSAGE.get("failed_to_create_account")})),
))]
/// 新しい口座情報を作成するAPI
/// # Arguments
/// * `pool` - Sqliteのコネクションプール
/// * `app_module` - アプリケーションのDIコンテナ
/// * `data` - リクエストボディから取得した口座情報
pub async fn create_account_handler(
    pool: web::Data<SqlitePool>,
    app_module: web::Data<AppModule>,
    data: web::Json<Account>,
) -> Result<HttpResponse, actix_web::Error> {
    let handler: Arc<dyn AccountHandler> = app_module.resolve();
    handler.create_account(pool, data).await
}

#[utoipa::path(delete, path = "/api/account/{id}", tag = "accounts", params(
    ("id" = String, Path, description = "Account ID(UUID)")
), responses(
    (status = 204, description = "Account deleted successfully"),
    (status = 404, description = "Account not found", body = serde_json::Value, example = json!({"error": MESSAGE.get("account_not_found")})),
    (status = 500, description = "Internal server error", body = serde_json::Value, example = json!({"error": MESSAGE.get("failed_to_delete_account")})),
))]
/// 指定されたIDの口座情報を削除するAPI
/// # Arguments
/// * `pool` - Sqliteのコネクションプール
/// * `app_module` - アプリケーションのDIコンテナ
/// * `path` - リクエストパスから取得した口座ID
/// # Returns
/// 成功時はHTTP 204、失敗時はHTTP 500とエラーメッセージを返す
pub async fn delete_account_handler(
    pool: web::Data<SqlitePool>,
    app_module: web::Data<AppModule>,
    path: web::Path<String>,
) -> Result<HttpResponse, actix_web::Error> {
    let handler: Arc<dyn AccountHandler> = app_module.resolve();
    handler.delete_account(pool, path).await
}

#[utoipa::path(get, path = "/api/account/{id}", tag = "accounts", params(
    ("id" = String, Path, description = "Account ID(UUID)")
), responses(
    (status = 200, description = "Account fetched successfully", body = Account),
    (status = 404, description = "Account not found", body = serde_json::Value, example = json!({"error": MESSAGE.get("account_not_found")})),
    (status = 500, description = "Internal server error", body = serde_json::Value, example = json!({"error": MESSAGE.get("failed_to_fetch_account")})),
))]
/// 指定されたIDの口座情報を取得するAPI
/// # Arguments
/// * `pool` - Sqliteのコネクションプール
/// * `app_module` - アプリケーションのDIコンテナ
/// * `path` - リクエストパスから取得した口座ID
/// # Returns
/// 成功時はHTTP 200、失敗時はHTTP 500とエラーメッセージを返す
pub async fn get_account_by_id_handler(
    pool: web::Data<SqlitePool>,
    app_module: web::Data<AppModule>,
    path: web::Path<String>,
) -> Result<HttpResponse, actix_web::Error> {
    let handler: Arc<dyn AccountHandler> = app_module.resolve();
    handler.get_account_by_id(pool, path).await
}

#[utoipa::path(get, path = "/api/account", tag = "accounts", responses(
    (status = 200, description = "Accounts fetched successfully", body = [Account]),
    (status = 500, description = "Internal server error", body = serde_json::Value, example = json!({"error": MESSAGE.get("failed_to_fetch_accounts")}))
))]
/// 全ての口座情報を取得するAPI
/// # Arguments
/// * `pool` - Sqliteのコネクションプール
/// * `app_module` - アプリケーションのDIコンテナ
/// # Returns
/// 成功時はHTTP 200、失敗時はHTTP 500とエラーメッセージを返す
pub async fn get_accounts_list_all_handler(
    pool: web::Data<SqlitePool>,
    app_module: web::Data<AppModule>,
) -> Result<HttpResponse, actix_web::Error> {
    let handler: Arc<dyn AccountHandler> = app_module.resolve();
    handler.get_accounts_list_all(pool).await
}

#[utoipa::path(get, path = "/api/account/type/{type_name}", tag = "accounts", params(
    ("type_name" = String, Path, description = "Account Type Name")
), responses(
    (status = 200, description = "Accounts fetched successfully", body = [Account]),
    (status = 404, description = "No accounts found", body = serde_json::Value, example = json!({"error": MESSAGE.get("no_accounts_found")})),
    (status = 500, description = "Internal server error", body = serde_json::Value, example = json!({"error": MESSAGE.get("failed_to_fetch_accounts")})),
))]
/// 指定された口座種別の口座情報を取得するAPI
/// # Arguments
/// * `pool` - Sqliteのコネクションプール
/// * `app_module` - アプリケーションのDIコンテナ
/// * `path` - リクエストパスから取得した口座種別名
/// # Returns
/// 成功時はHTTP 200、失敗時はHTTP 500とエラーメッセージを返す
pub async fn get_accounts_list_by_type_handler(
    pool: web::Data<SqlitePool>,
    app_module: web::Data<AppModule>,
    path: web::Path<String>,
) -> Result<HttpResponse, actix_web::Error> {
    let handler: Arc<dyn AccountHandler> = app_module.resolve();
    handler.get_accounts_list_by_type(pool, path).await
}

#[utoipa::path(put, path = "/api/account", tag = "accounts", request_body = Account, responses(
    (status = 200, description = "Account updated successfully", body = serde_json::Value, example = json!({"updated": 1})),
    (status = 404, description = "Account not found", body = serde_json::Value, example = json!({"error": MESSAGE.get("account_not_found")})),
    (status = 500, description = "Internal server error", body = serde_json::Value, example = json!({"error": MESSAGE.get("failed_to_update_account")})),
))]
/// 指定されたIDの口座情報を更新するAPI
/// # Arguments
/// * `pool` - Sqliteのコネクションプール
/// * `app_module` - アプリケーションのDIコンテナ
/// * `data` - リクエストボディから取得した口座情報
/// # Returns
/// 成功時はHTTP 200、失敗時はHTTP 500とエラーメッセージを返す
pub async fn update_account_handler(
    pool: web::Data<SqlitePool>,
    app_module: web::Data<AppModule>,
    data: web::Json<Account>,
) -> Result<HttpResponse, actix_web::Error> {
    let handler: Arc<dyn AccountHandler> = app_module.resolve();
    handler.update_account(pool, data).await
}

/// 口座情報ハンドラーのインターフェース
#[async_trait::async_trait]
pub trait AccountHandler: Interface {
    async fn create_account(
        &self,
        pool: web::Data<SqlitePool>,
        data: web::Json<Account>,
    ) -> Result<HttpResponse, actix_web::Error>;
    async fn delete_account(
        &self,
        pool: web::Data<SqlitePool>,
        path: web::Path<String>,
    ) -> Result<HttpResponse, actix_web::Error>;
    async fn get_account_by_id(
        &self,
        pool: web::Data<SqlitePool>,
        path: web::Path<String>,
    ) -> Result<HttpResponse, actix_web::Error>;
    async fn get_accounts_list_all(&self, pool: web::Data<SqlitePool>) -> Result<HttpResponse, actix_web::Error>;
    async fn get_accounts_list_by_type(
        &self,
        pool: web::Data<SqlitePool>,
        path: web::Path<String>,
    ) -> Result<HttpResponse, actix_web::Error>;
    async fn update_account(
        &self,
        pool: web::Data<SqlitePool>,
        data: web::Json<Account>,
    ) -> Result<HttpResponse, actix_web::Error>;
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
    async fn create_account(
        &self,
        pool: web::Data<SqlitePool>,
        data: web::Json<Account>,
    ) -> Result<HttpResponse, actix_web::Error> {
        info!("Received request to create a new account");
        let account_id = data.id.clone();
        // IDの重複チェック
        let result = self.account_dao.get_account_by_id(&pool, &account_id).await;
        match result {
            Ok(account) => {
                if account.is_some() {
                    info!("Account with ID {} already exists.", account_id);
                    return Ok(
                        HttpResponse::Conflict().json(json!({"error": MESSAGE.get("account_id_already_exists")}))
                    );
                }
            },
            Err(e) => {
                error!("Database error: {:?}", e);
                return Ok(
                    HttpResponse::InternalServerError().json(json!({"error": MESSAGE.get("failed_to_fetch_account")}))
                );
            },
        }
        // 口座の作成
        let result = self.account_dao.create_account(&pool, &data).await;
        match result {
            Ok(success_count) => {
                info!("Created {} new account(s).", success_count);
                Ok(HttpResponse::Created().json(json!({"created": success_count})))
            },
            Err(e) => {
                error!("Database error: {:?}", e);
                Ok(HttpResponse::InternalServerError().json(json!({"error": MESSAGE.get("failed_to_create_account")})))
            },
        }
    }

    /// 指定されたIDの口座情報を削除するハンドラー関数
    /// # Arguments
    /// * `pool` - データベース接続プール
    /// * `path` - URLパスから取得した口座ID
    /// # Returns
    /// 成功時はHTTP 200と成功メッセージ、失敗時はHTTP 500とエラーメッセージ
    async fn delete_account(
        &self,
        pool: web::Data<SqlitePool>,
        path: web::Path<String>,
    ) -> Result<HttpResponse, actix_web::Error> {
        let account_id = path.as_str();
        info!("Received request to delete account by ID: {}", account_id);
        let result = self.account_dao.delete_account(&pool, account_id).await;
        match result {
            Ok(del_count) => {
                if del_count == 0 {
                    info!("No account found with ID: {}", account_id);
                    return Ok(HttpResponse::NotFound().json(json!({"error": MESSAGE.get("account_not_found")})));
                }
                info!("Deleted account with ID: {}", account_id);
                Ok(HttpResponse::NoContent().finish())
            },
            Err(e) => {
                error!("Database error: {:?}", e);
                Ok(HttpResponse::InternalServerError().json(json!({"error": MESSAGE.get("failed_to_delete_account")})))
            },
        }
    }

    /// 指定されたIDの口座情報を取得するハンドラー関数
    /// # Arguments
    /// * `pool` - データベース接続プール
    /// * `path` - URLパスから取得した口座ID
    /// # Returns
    /// 成功時はHTTP 200と口座情報のJSON、失敗時はHTTP 500とエラーメッセージ
    async fn get_account_by_id(
        &self,
        pool: web::Data<SqlitePool>,
        path: web::Path<String>,
    ) -> Result<HttpResponse, actix_web::Error> {
        let account_id = path.as_str();
        info!("Received request to fetch account by ID: {}", account_id);
        let result = self.account_dao.get_account_by_id(&pool, account_id).await;
        match result {
            Ok(account) => {
                if account.is_none() {
                    info!("No account found with ID: {}", account_id);
                    return Ok(HttpResponse::NotFound().json(json!({"error": MESSAGE.get("account_not_found")})));
                }
                info!("Fetched account with ID: {}", account_id);
                Ok(HttpResponse::Ok().json(account))
            },
            Err(e) => {
                error!("Database error: {:?}", e);
                Ok(HttpResponse::InternalServerError().json(json!({"error": MESSAGE.get("failed_to_fetch_account")})))
            },
        }
    }

    /// 全ての口座情報を取得するハンドラー関数
    /// # Arguments
    /// * `pool` - データベース接続プール
    /// # Returns
    /// 成功時はHTTP 200と口座情報のJSON配列、失敗時はHTTP 500とエラーメッセージ
    async fn get_accounts_list_all(&self, pool: web::Data<SqlitePool>) -> Result<HttpResponse, actix_web::Error> {
        info!("Received request to fetch all accounts");
        let result = self.account_dao.get_accounts_list_all(&pool).await;
        match result {
            Ok(accounts) => {
                info!("Fetched {} accounts.", accounts.len());
                Ok(HttpResponse::Ok().json(accounts))
            },
            Err(e) => {
                error!("Database error: {:?}", e);
                Ok(HttpResponse::InternalServerError().json(json!({"error": MESSAGE.get("failed_to_fetch_accounts")})))
            },
        }
    }

    /// 指定された口座種別の口座情報を取得するハンドラー関数
    /// # Arguments
    /// * `pool` - データベース接続プール
    /// * `type_name` - URLパスから取得した口座種別名
    /// # Returns
    /// 成功時はHTTP 200と口座情報のJSON配列、失敗時はHTTP 500とエラーメッセージ
    async fn get_accounts_list_by_type(
        &self,
        pool: web::Data<SqlitePool>,
        type_name: web::Path<String>,
    ) -> Result<HttpResponse, actix_web::Error> {
        info!("Received request to fetch accounts of type: {}", type_name);
        let result = self.account_dao.get_accounts_list_by_type(&pool, &type_name).await;
        match result {
            Ok(accounts) => {
                if accounts.is_empty() {
                    info!("No accounts found for type: {}", type_name);
                    return Ok(HttpResponse::NotFound().json(json!({"error": MESSAGE.get("no_accounts_found")})));
                }
                info!("Fetched {} accounts of type: {}", accounts.len(), type_name);
                Ok(HttpResponse::Ok().json(accounts))
            },
            Err(e) => {
                error!("Database error: {:?}", e);
                Ok(HttpResponse::InternalServerError().json(json!({"error": MESSAGE.get("failed_to_fetch_accounts")})))
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
        data: web::Json<Account>,
    ) -> Result<HttpResponse, actix_web::Error> {
        info!("Received request to update account by ID: {}", data.id);
        let result = self.account_dao.update_account(&pool, &data).await;
        match result {
            Ok(updated_count) => {
                if updated_count == 0 {
                    info!("No account found with ID: {}", data.id);
                    return Ok(HttpResponse::NotFound().json(json!({"error": MESSAGE.get("account_not_found")})));
                }
                info!("Updated account with ID: {}", data.id);
                Ok(HttpResponse::Ok().json(json!({"updated": updated_count})))
            },
            Err(e) => {
                error!("Database error: {:?}", e);
                Ok(HttpResponse::InternalServerError().json(json!({"error": MESSAGE.get("failed_to_update_account")})))
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::unit_test::fixtures::accounts as fixtures_accounts;
    use crate::utils::unit_test::fixtures::accounts::{
        MockConfig, ParametrizedMockAccountDaoImpl, ParametrizedMockAccountDaoImplParameters,
    };
    use crate::utils::unit_test::fixtures::app_test_setup::TestAppModule;
    use crate::utils::unit_test::fixtures::db as fixtures_db;
    use actix_web::body::to_bytes;
    use actix_web::http::StatusCode;

    async fn setup(parameters: MockConfig) -> (web::Data<SqlitePool>, Arc<dyn AccountHandler>) {
        let pool = web::Data::new(fixtures_db::create_undefined_db().await);
        let add_module = TestAppModule::builder()
            .with_component_parameters::<ParametrizedMockAccountDaoImpl>(ParametrizedMockAccountDaoImplParameters {
                config: Arc::new(parameters),
            })
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
            let resp = _handler.create_account(pool, existing_account_json).await.unwrap();
            // assertion
            assert_eq!(resp.status(), StatusCode::CONFLICT);
            let body_bytes = to_bytes(resp.into_body()).await.unwrap();
            let body: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
            assert_eq!(body, json!({"error": "Account ID already exists."}));
        }

        #[tokio::test]
        async fn create_servererror_on_check_exists() {
            // preparation
            let (pool, _handler) = setup(get_error_params()).await;
            let new_account = fixtures_accounts::get_first_account();
            let new_account_json = web::Json(new_account.clone());
            // execution
            let resp = _handler.create_account(pool, new_account_json).await.unwrap();
            // assertion
            assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);
            let body_bytes = to_bytes(resp.into_body()).await.unwrap();
            let body: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
            assert_eq!(body, json!({"error": "Failed to fetch account."}));
        }

        // TODO: モックを使えるようになってから
        #[tokio::test]
        async fn create_servererror_on_creation() {
            // preparation
            let (pool, _handler) = setup(get_error_on_creation_params()).await;
            let new_account = fixtures_accounts::get_first_account();
            let new_account_json = web::Json(new_account.clone());
            // execution
            let resp = _handler.create_account(pool, new_account_json).await.unwrap();
            // assertion
            assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);
            let body_bytes = to_bytes(resp.into_body()).await.unwrap();
            let body: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
            assert_eq!(body, json!({"error": "Failed to create account."}));
        }
    }

    mod get_accounts_list_all {
        use super::*;

        #[tokio::test]
        async fn get_all_is_empty() {
            // preparation
            let (pool, _handler) = setup(get_empty_params()).await;
            // execution
            let resp = _handler.get_accounts_list_all(pool).await.unwrap();
            // assertion
            assert_eq!(resp.status(), StatusCode::OK);
            let body_bytes = to_bytes(resp.into_body()).await.unwrap();
            let accounts: Vec<Account> = serde_json::from_slice(&body_bytes).unwrap();
            assert_eq!(accounts.len(), 0);
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
            let resp = _handler.get_accounts_list_all(pool).await.unwrap();
            // assertion
            assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);
            let body_bytes = to_bytes(resp.into_body()).await.unwrap();
            let body: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
            assert_eq!(body, json!({"error": "Failed to fetch accounts."}));
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
            let resp = _handler.get_accounts_list_by_type(pool, path).await.unwrap();
            // assertion
            assert_eq!(resp.status(), StatusCode::NOT_FOUND);
            let body_bytes = to_bytes(resp.into_body()).await.unwrap();
            let body: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
            assert_eq!(body, json!({"error": "No accounts found."}));
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
            let resp = _handler.get_accounts_list_by_type(pool, path).await.unwrap();
            // assertion
            assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);
            let body_bytes = to_bytes(resp.into_body()).await.unwrap();
            let body: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
            assert_eq!(body, json!({"error": "Failed to fetch accounts."}));
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
            let resp = _handler.get_account_by_id(pool, path).await.unwrap();
            // assertion
            assert_eq!(resp.status(), StatusCode::NOT_FOUND);
            let body_bytes = to_bytes(resp.into_body()).await.unwrap();
            let body: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
            assert_eq!(body, json!({"error": "Account not found."}));
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
            let resp = _handler.get_account_by_id(pool, path).await.unwrap();
            // assertion
            assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);
            let body_bytes = to_bytes(resp.into_body()).await.unwrap();
            let body: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
            assert_eq!(body, json!({"error": "Failed to fetch account."}));
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
            let resp = _handler.delete_account(pool, path).await.unwrap();
            // assertion
            assert_eq!(resp.status(), StatusCode::NOT_FOUND);
            let body_bytes = to_bytes(resp.into_body()).await.unwrap();
            let body: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
            assert_eq!(body, json!({"error": "Account not found."}));
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
            let resp = _handler.delete_account(pool, path).await.unwrap();
            // assertion
            assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);
            let body_bytes = to_bytes(resp.into_body()).await.unwrap();
            let body: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
            assert_eq!(body, json!({"error": "Failed to delete account."}));
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
            let resp = _handler.update_account(pool, account_json).await.unwrap();
            // assertion
            assert_eq!(resp.status(), StatusCode::NOT_FOUND);
            let body_bytes = to_bytes(resp.into_body()).await.unwrap();
            let body: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
            assert_eq!(body, json!({"error": "Account not found."}));
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
            let resp = _handler.update_account(pool, account_json).await.unwrap();
            // assertion
            assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);
            let body_bytes = to_bytes(resp.into_body()).await.unwrap();
            let body: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
            assert_eq!(body, json!({"error": "Failed to update account."}));
        }
    }
}
