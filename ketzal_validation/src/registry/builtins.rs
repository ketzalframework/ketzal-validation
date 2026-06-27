use crate::registry::ValidatorRegistry;
use crate::validators::common::max::Max;
use crate::validators::common::min::Min;

pub fn register_builtins(registry: &mut ValidatorRegistry) {
    registry.register(Max);
    registry.register(Min);
}
