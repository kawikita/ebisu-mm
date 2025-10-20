include!(concat!(env!("CARGO_MANIFEST_DIR"), "/build_number.rs"));

/// バージョン情報を出力する関数
/// # Arguments
/// * `verbose` - 詳細情報を表示するかどうか
pub fn show_version(verbose: bool) {
    // Cargo.tomlのバージョン情報を埋め込む
    println!(
        "{name} v{version}",
        name = env!("CARGO_PKG_NAME"),
        version = format!("{}.{}", env!("CARGO_PKG_VERSION"), BUILD_NUMBER),
    );
    if verbose {
        show_version_with_verbose();
    }
}

// 詳細なバージョン情報を出力する関数
fn show_version_with_verbose() {
    println!(
        "\nDescription: {desc}\nRust Edition: {edition}\nAuthor(s):\n  {authors}",
        desc = env!("CARGO_PKG_DESCRIPTION"),
        edition = EDITION,
        authors = env!("CARGO_PKG_AUTHORS"),
    );
}
