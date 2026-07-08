pub mod negative;
pub mod positive;

use crate::registry::ValidatorRegistry;

pub(crate) fn register(registry: &mut ValidatorRegistry) {
    registry.register(negative::Negative);
    registry.register(positive::Positive);
}
