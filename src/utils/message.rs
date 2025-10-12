use log::error;
use once_cell::sync::OnceCell;
use serde_yaml::Value;
use std::collections::HashMap;
use strfmt::strfmt;

// --- バイナリ埋め込み ---
// build.rsで生成したRust静的変数をinclude!で取り込む
include!(concat!(env!("OUT_DIR"), "/messages_embedded.rs"));

static MESSAGES: OnceCell<Value> = OnceCell::new();

fn load_messages() -> Value {
    serde_yaml::from_str(MESSAGES_YAML).expect("Failed to parse embedded messages.yaml")
}

fn get_messages_root() -> &'static Value {
    MESSAGES.get_or_init(load_messages)
}

pub struct MessageHierarchy<'a> {
    node: &'a Value,
}

impl<'a> MessageHierarchy<'a> {
    /// メッセージを取得。未定義時は空文字列＋エラーログ
    pub fn get(&self, key: &str) -> &str {
        match self.node.get(key).and_then(|v| v.as_str()) {
            Some(s) => s,
            None => {
                error!("Message key '{}' found but not a string", key);
                ""
            },
        }
    }

    /// テンプレート文字列にstrfmtで埋め込み。失敗時は空文字＋エラーログ
    pub fn get_fmt(&self, key: &str, vars: &HashMap<String, String>) -> String {
        let template = self.get(key);
        match strfmt(template, vars) {
            Ok(s) => s,
            Err(e) => {
                error!("Failed to format message for key '{}': {}", key, e);
                "".to_string()
            },
        }
    }
}

/// module_path!() で与えられたパスに対応するメッセージサブツリーを返す。
/// 階層のYAMLが存在しない場合panic。
pub fn set_hierarchy(module_path: &str) -> MessageHierarchy<'_> {
    let root = get_messages_root();
    let mut node = root;
    for part in module_path.split("::") {
        match node.get(part) {
            Some(next) => node = next,
            None => panic!(
                "Message YAML for module_path '{}' not found (missing part '{}'). Ensure the corresponding YAML file or section exists. See the documentation for message configuration.",
                module_path, part
            ),
        }
    }
    MessageHierarchy { node }
}

#[macro_export]
macro_rules! msg_map {
    ( $( $k:expr => $v:expr ),* $(,)? ) => {{
        let mut m = std::collections::HashMap::new();
        $( m.insert($k.to_string(), $v.to_string()); )*
        m
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_messages_root() {
        let root = get_messages_root();
        assert!(matches!(root, Value::Mapping(_)));
    }

    #[test]
    fn test_set_hierarchy() {
        let hierarchy = set_hierarchy("common::test");
        assert!(matches!(hierarchy.node, Value::Mapping(_)));
    }

    #[test]
    fn test_get() {
        let h: MessageHierarchy<'_> = set_hierarchy("common::test");
        assert_eq!(h.get("simple"), "Hello, world!");
    }

    #[test]
    fn test_get_fmt() {
        let h = set_hierarchy("common::test");
        let mut vars = std::collections::HashMap::new();
        vars.insert("name".to_string(), "Taro".to_string());
        let s = h.get_fmt("greet", &vars);
        assert_eq!(s, "Hello, Taro!".to_string());
    }
}
