use std::collections::HashMap;
use std::sync::OnceLock;

use crate::traits::Validator;

pub struct ValidatorRegistry {
    validators: HashMap<String, Box<dyn Validator>>,
}

impl ValidatorRegistry {
    pub fn new() -> Self {
        Self {
            validators: HashMap::new(),
        }
    }

    pub fn register<V>(&mut self, validator: V)
    where
        V: Validator + 'static,
    {
        self.validators
            .insert(validator.name().to_string(), Box::new(validator));
    }

    pub fn get(&self, name: &str) -> Option<&dyn Validator> {
        self.validators.get(name).map(|b| b.as_ref())
    }

    pub fn global() -> &'static Self {
        static GLOBAL: OnceLock<ValidatorRegistry> = OnceLock::new();
        GLOBAL.get_or_init(|| {
            let mut registry = ValidatorRegistry::new();
            super::builtins::register_builtins(&mut registry);
            registry
        })
    }
}

impl Default for ValidatorRegistry {
    fn default() -> Self {
        Self::new()
    }
}
