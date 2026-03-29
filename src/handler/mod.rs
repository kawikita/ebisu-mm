use crate::utils::error::AppError;
use actix_web::HttpResponse;

/// 全ハンドラー共通の返却型エイリアス
pub type HandlerResult = Result<HttpResponse, AppError>;

pub mod accounts; // 口座関連のハンドラー
pub mod swagger_ui; // Swagger UI関連のハンドラー
