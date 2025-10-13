/// アプリケーションのコマンドオプションのパーサー
/// アプリケーションのコマンドオプションを格納する構造体
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "Ebisu API")]
#[command(about = "Ebisu API Server", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(about = "Export OpenAPI specification in JSON format")]
    Export {
        #[arg(short, long, help = "Output file path")]
        file: String,
    },
    #[command(about = "Show version information")]
    Version {
        #[arg(short, long, help = "Show detailed version information")]
        verbose: bool,
    },
}
