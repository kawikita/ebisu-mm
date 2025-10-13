use crate::handler::swagger_ui::ApiDoc;
use std::fs;
use std::io::{self, Write};
use utoipa::OpenApi;

/// OpenAPI仕様をJSON形式で指定されたファイルにエクスポートする関数
pub fn export_openapi_json(file_path: &str) -> io::Result<()> {
    let openapi = ApiDoc::openapi();
    let json = serde_json::to_string_pretty(&openapi).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    if let Some(parent) = std::path::Path::new(&file_path).parent() {
        fs::create_dir_all(parent)?;
    }
    let mut file = fs::File::create(file_path)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}
