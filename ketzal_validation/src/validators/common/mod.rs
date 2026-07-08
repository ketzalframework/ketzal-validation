pub mod max;
pub mod min;

use crate::registry::ValidatorRegistry;

pub(crate) fn register(registry: &mut ValidatorRegistry) {
    registry.register(min::Min);
    registry.register(max::Max);
}
