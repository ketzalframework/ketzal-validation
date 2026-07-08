use std::sync::RwLock;

use serde::Deserialize;

#[derive(Deserialize)]
struct AppConfig {
    locale: String,
}

static CONFIG: RwLock<Option<AppConfig>> = RwLock::new(None);

fn load_default() -> AppConfig {
    serde_yaml::from_str(include_str!("../../ketzal.yml")).expect("Failed to parse ketzal.yml")
}

pub fn locale() -> String {
    let guard = CONFIG.read().unwrap();
    match guard.as_ref() {
        Some(cfg) => cfg.locale.clone(),
        None => {
            drop(guard);
            let mut guard = CONFIG.write().unwrap();
            if guard.is_none() {
                *guard = Some(load_default());
            }
            guard.as_ref().unwrap().locale.clone()
        }
    }
}

pub fn set_locale(locale: impl Into<String>) {
    let mut guard = CONFIG.write().unwrap();
    match guard.as_mut() {
        Some(cfg) => cfg.locale = locale.into(),
        None => {
            let mut cfg = load_default();
            cfg.locale = locale.into();
            *guard = Some(cfg);
        }
    }
}
