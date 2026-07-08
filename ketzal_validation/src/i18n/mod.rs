use std::collections::HashMap;
use std::sync::OnceLock;

use crate::config;

type FlatMap = HashMap<String, String>;

static CACHE: OnceLock<(FlatMap, FlatMap)> = OnceLock::new();

fn flatten(value: &serde_json::Value, prefix: &str, map: &mut FlatMap) {
    match value {
        serde_json::Value::Object(obj) => {
            for (k, v) in obj {
                let key = if prefix.is_empty() {
                    k.clone()
                } else {
                    format!("{prefix}.{k}")
                };
                flatten(v, &key, map);
            }
        }
        serde_json::Value::String(s) => {
            map.insert(prefix.to_string(), s.clone());
        }
        _ => {}
    }
}

fn load() -> &'static (FlatMap, FlatMap) {
    CACHE.get_or_init(|| {
        let es: serde_json::Value =
            serde_json::from_str(include_str!("../../locales/es.json")).unwrap();
        let en: serde_json::Value =
            serde_json::from_str(include_str!("../../locales/en.json")).unwrap();

        let mut es_map = FlatMap::new();
        let mut en_map = FlatMap::new();
        flatten(&es, "", &mut es_map);
        flatten(&en, "", &mut en_map);

        (es_map, en_map)
    })
}

pub fn t(key: &str, params: &[(&str, &str)]) -> String {
    t_with(&config::locale(), key, params)
}

pub fn t_with(lang: &str, key: &str, params: &[(&str, &str)]) -> String {
    let (es, en) = load();

    let template = match lang {
        "en" => en.get(key).or_else(|| es.get(key)).map(String::as_str),
        _ => es.get(key).map(String::as_str),
    }
    .unwrap_or(key);

    let mut msg = template.to_string();
    for (k, v) in params {
        msg = msg.replace(&format!("{{{k}}}"), v);
    }
    msg
}
