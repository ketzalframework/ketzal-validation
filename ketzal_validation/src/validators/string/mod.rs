pub mod alpha;
pub mod email;
pub mod regex;

use crate::registry::ValidatorRegistry;

pub(crate) fn register(registry: &mut ValidatorRegistry) {
    registry.register(alpha::Alpha);
    registry.register(email::Email);
    registry.register(regex::RegexValidator);
}
