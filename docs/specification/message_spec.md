
# メッセージ管理仕様書

## 目的
本ドキュメントは、Ebisu APIにおけるメッセージ管理の設計方針と運用ルー＿を定め、保守性・拡張性・運用性の高いメッセージ管理を実現することを目的とする。

## ディレクトリ構成
- `messages/` ディレクトリ配下に、Rustのモジュール階層と同じ構造でYAMLファイルを配置する。
    - 例: `dao/accounts.rs` → `messages/dao/accounts.yaml`

## メッセージ定義
- 各YAMLファイルにメッセージを定義する。
    - YAMLのキーをメッセージの識別子とする。
        - 例:
            ```yaml
            account_not_found: "Account not found."
            # フォーマット用の値を埋め込む場合は、必ず名前付きプレースホルダ（{name}など）を使うこと。
            fetched_n_accounts: "Fetched {count} accounts."
            user_greeting: "Hello, {name}!"
            ```
        - プレースホルダは必ず `{name}` のような「名前付き」とし、呼び出し時はHashMapで値を渡すこと。

## Rustコードからの利用方法
- モジュールパスを指定してメッセージ階層を取得する。
    - 例:
      ```rust
      let msg = messages::set_hierarchy(module_path!());
      ```
- 階層のYAMLファイルが存在しない場合はpanicとし、起動不可とする。
    - 理由: メッセージ出力に支障が出るため、初期不良を早期発見する。

## メッセージ取得API

- メッセージを取得する:
        - 例:
            ```rust
            let s = msg.get("fetched_n_accounts");
            ```

- メッセージ取得時に引数（名前付き）をmsg_map!マクロで渡し、フォーマット済み文字列を取得する:
        - 例:
            ```rust
            let s = msg.get_fmt("fetched_n_accounts", &msg_map!("count" => 10));
            let s = msg.get_fmt("fetch_n_accounts_with_type_id", &msg_map!("count" => 10, "account_type_id" => "bank"));
            ```
        - プレースホルダは必ず `{name}` のような「名前付き」とし、msg_map!マクロで複数ペアを簡潔に渡せる。
        - 引数は可変長で、DBの値など動的なものも許容（コンパイル時に決まっている必要はない）。

### msg_map!マクロについて
- 使い方: `msg_map!("key1" => val1, "key2" => val2, ...)`
- すべての値はto_string()でString化されるため、数値や&strもそのまま渡せる
- 複数ペアを一度に指定でき、どのモジュールからも呼び出し可能

## メッセージのバイナリ埋め込み
- YAMLファイルのメッセージはコンパイル時にバイナリに埋め込む。
    - 例: `include_str!`や`build.rs`等を利用。

## エラー時の挙動
- メッセージが未定義の場合は空文字列を返し、errorログを出力する。
- メッセージのフォーマットに失敗した場合も空文字列を返し、errorログを出力する。

## 補足

- メッセージはREST APIの一貫性維持のため英語のみで管理・運用する。
    - 多言語化するとクライアント側の対応も必要になりコストがかかるため。
    - 多言語対応はフロント側に一任する。
- 実装時はロガーの統一やテスト容易性も考慮すること。
