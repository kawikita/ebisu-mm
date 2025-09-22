use env_logger::{Builder, Target};
use log::{info, LevelFilter};
use std::{env, io::Write, fs::OpenOptions, path::Path};
use chrono;

/// ロガーの初期化
/// ログファイルのパスは環境変数 LOG_FILE_PATH で指定
/// 指定がない場合は ./logs/ebisu_api.log に出力
pub fn init_logger() {
    let log_file_path: String = env::var("LOG_FILE_PATH").unwrap_or_else(|_| "./logs/ebisu_api.log".to_string());
    let mut builder: Builder = Builder::from_default_env();
    set_log_message_format(&mut builder);
    builder.target(open_log_file(&log_file_path));
    builder.init();
    info!("Logger initialized");
}

// ログメッセージのフォーマットを設定するヘルパー関数
fn set_log_message_format(builder: &mut Builder) {
    builder.filter_level(LevelFilter::Debug)
        .format(|buf, record| {
            writeln!(
                buf,
                "[{}][{}] {} - {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.9f"),
                record.level(),
                record.module_path().unwrap_or("<unknown>"),
                record.args()
            )
        });
}

// ログファイルを開くヘルパー関数
fn open_log_file(path: &str) -> Target {
    // ログファイルのディレクトリが存在しない場合は作成する
    let log_path = Path::new(path);
    if let Some(parent) = log_path.parent() {
        if let Err(e) = std::fs::create_dir_all(parent) {
            eprintln!("Warning: Failed to create log directory {}: {}", parent.display(), e);
            return Target::Stderr;
        }
    }
    // ログファイルを開く
    match OpenOptions::new().create(true).append(true).open(log_path) {
        Ok(file) => {
            info!("Logging to file: {}", path);
            Target::Pipe(Box::new(file))
        }
        Err(e) => {
            eprintln!("Warning: Failed to open log file at {}: {}", path, e);
            Target::Stderr
        }
    }
}