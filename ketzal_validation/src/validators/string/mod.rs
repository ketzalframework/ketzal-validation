pub mod email;
pub mod regex;

use crate::registry::ValidatorRegistry;

pub(crate) fn register(registry: &mut ValidatorRegistry) {
    registry.register(email::Email);
    registry.register(regex::RegexValidator);
}
