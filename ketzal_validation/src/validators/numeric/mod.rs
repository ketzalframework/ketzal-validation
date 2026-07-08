pub mod between;
pub mod integer;
pub mod negative;
pub mod positive;

use crate::registry::ValidatorRegistry;

pub(crate) fn register(registry: &mut ValidatorRegistry) {
    registry.register(between::Between);
    registry.register(integer::Integer);
    registry.register(negative::Negative);
    registry.register(positive::Positive);
}
