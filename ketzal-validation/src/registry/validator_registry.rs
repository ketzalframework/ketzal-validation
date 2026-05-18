use std::collections::HashMap;

use crate::traits::validator::Validator;

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

    pub fn get(&self, name: &str) -> Option<&Box<dyn Validator>> {
        self.validators.get(name)
    }
}
