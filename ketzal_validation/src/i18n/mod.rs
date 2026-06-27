use std::collections::HashMap;
use std::sync::OnceLock;

use crate::config;

type LangMap = HashMap<String, String>;

static CACHE: OnceLock<HashMap<&'static str, LangMap>> = OnceLock::new();

fn load() -> &'static HashMap<&'static str, LangMap> {
    CACHE.get_or_init(|| {
        let mut map: HashMap<&str, LangMap> = HashMap::new();
        map.insert(
            "es",
            serde_json::from_str(include_str!("../../locales/es.json")).unwrap(),
        );
        map.insert(
            "en",
            serde_json::from_str(include_str!("../../locales/en.json")).unwrap(),
        );
        map
    })
}

pub fn t(key: &str, params: &[(&str, &str)]) -> String {
    t_with(&config::locale(), key, params)
}

pub fn t_with(lang: &str, key: &str, params: &[(&str, &str)]) -> String {
    let all = load();

    let lang_map = all
        .get(lang)
        .or_else(|| all.get("es"))
        .expect("Default language 'es' not found");

    let template = lang_map.get(key).map(String::as_str).unwrap_or(key);

    let mut msg = template.to_string();
    for (k, v) in params {
        msg = msg.replace(&format!("{{{k}}}"), v);
    }
    msg
}
