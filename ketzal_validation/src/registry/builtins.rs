use crate::registry::ValidatorRegistry;
use crate::validators;

pub fn register_builtins(registry: &mut ValidatorRegistry) {
    validators::common::register(registry);
    validators::numeric::register(registry);
}
