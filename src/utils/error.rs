use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use log::error;
use serde::Serialize;
use thiserror::Error;
use utoipa::ToSchema;

/// アプリケーション全体で使用するエラー型とエラーレスポンスの定義
#[derive(Serialize, ToSchema, Debug)]
pub struct ErrorResponse {
    error: ErrorDetail,
}

/// エラーの詳細情報を表す構造体
#[derive(Serialize, ToSchema, Debug)]
pub struct ErrorDetail {
    code: u16,
    message: String,
}

impl AppError {
    /// AppErrorからErrorResponseを生成するためのヘルパー関数
    /// # Arguments
    /// * `self` - AppErrorのインスタンス
    /// # Returns
    /// * `ErrorResponse` - AppErrorの内容を反映したエラーレスポンス構造体
    pub fn to_error_response(&self) -> ErrorResponse {
        ErrorResponse {
            error: ErrorDetail {
                code: self.status_code().as_u16(),
                message: self.to_string(),
            },
        }
    }
}

/// アプリケーション全体で使用するエラー型の定義
/// # Errors
/// - `BadRequest`: クライアントからのリクエストが不正な場合に使用
/// - `Unauthorized`: 認証が必要なリソースに対して認証されていないアクセスがあった場合に使用
/// - `NotFound`: リクエストされたリソースが見つからない場合に使用
/// - `Conflict`: リクエストが現在のリソースの状態と競合する場合に使用
/// - `InternalServerError`: サーバー内部で予期しないエラーが発生した場合に使用
#[derive(Error, Debug)]
pub enum AppError {
    // HTTP Error
    #[error("{0}")] // 400 Bad Request
    BadRequest(String),

    #[error("{0}")] // 内部的には異なるが外部的にはBadRequest(400)にする
    InvalidInput(String),

    #[error("{0}")] // 401 Unauthorized
    Unauthorized(String),

    #[error("{0}")] // 404 Not Found
    NotFound(String),

    #[error("{0}")] // 409 Conflict
    Conflict(String),

    #[error("{0}")] // 500 Internal Server Error
    InternalServerError(String),
}

/// AppErrorをHTTPレスポンスに変換するための実装
/// 各エラータイプに対応するHTTPステータスコードを返し、エラーメッセージをログに記録してJSON形式でレスポンスを生成する
impl ResponseError for AppError {
    /// エラータイプに対応するHTTPステータスコードを返す
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::Conflict(_) => StatusCode::CONFLICT,
            AppError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            AppError::BadRequest(_) | AppError::InvalidInput(_) => StatusCode::BAD_REQUEST,
            AppError::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    /// エラーメッセージをログに記録し、JSON形式でHTTPレスポンスを生成する
    fn error_response(&self) -> HttpResponse {
        let status_code = self.status_code();
        let error_res = self.to_error_response();
        error!("Error occurred: {}", error_res.error.message);
        HttpResponse::build(status_code).json(error_res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_error_response() {
        testing_logger::setup();
        let error = AppError::NotFound(String::from("Resource not found"));
        let response = error.error_response();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
        testing_logger::validate(|captured_logs| {
            let error_logs: Vec<_> = captured_logs.iter().filter(|log| log.level == log::Level::Error).collect();
            assert_eq!(error_logs.len(), 1);
            let log_message = &error_logs[0].body;
            assert!(log_message.contains("Error occurred: Resource not found"));
        });
    }
}
