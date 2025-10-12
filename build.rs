use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

use serde_yaml::{Mapping, Value};

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("messages_embedded.rs");

    let mut root = Mapping::new();

    for entry in glob::glob("messages/**/*.yaml").unwrap() {
        let path = entry.unwrap();
        let value: Value = serde_yaml::from_str(&fs::read_to_string(&path).unwrap()).unwrap();
        let keys = path_to_keys(&path);
        insert_nested(&mut root, &keys, value);
    }

    let merged = Value::Mapping(root);
    let yaml_str = serde_yaml::to_string(&merged).unwrap();

    let mut f = File::create(&dest_path).unwrap();
    // Rustの静的変数として埋め込む
    writeln!(f, "pub const MESSAGES_YAML: &str = r#\"{}\"#;", yaml_str).unwrap();

    println!("cargo:rerun-if-changed=messages/");
}

fn path_to_keys(path: &PathBuf) -> Vec<String> {
    let mut keys = Vec::new();
    let mut iter = path.iter().skip_while(|c| *c != "messages");
    iter.next(); // skip "messages"
    for comp in iter {
        let s = comp.to_string_lossy();
        if s.ends_with(".yaml") {
            keys.push(s.trim_end_matches(".yaml").to_string());
        } else {
            keys.push(s.to_string());
        }
    }
    keys
}

fn insert_nested(map: &mut Mapping, keys: &[String], value: Value) {
    if keys.is_empty() {
        return;
    }
    if keys.len() == 1 {
        map.insert(Value::String(keys[0].clone()), value);
    } else {
        let entry = map.entry(Value::String(keys[0].clone())).or_insert_with(|| Value::Mapping(Mapping::new()));
        if let Value::Mapping(submap) = entry {
            insert_nested(submap, &keys[1..], value);
        }
    }
}
