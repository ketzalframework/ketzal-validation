pub mod alpha;
pub mod email;
pub mod r#in;
pub mod regex;

use crate::registry::ValidatorRegistry;

pub(crate) fn register(registry: &mut ValidatorRegistry) {
    registry.register(alpha::Alpha);
    registry.register(email::Email);
    registry.register(r#in::In);
    registry.register(r#in::NotIn);
    registry.register(regex::RegexValidator);
}
