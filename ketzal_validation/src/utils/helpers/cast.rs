pub(crate) fn any_to_f64(value: &dyn std::any::Any) -> Option<f64> {
    if let Some(v) = value.downcast_ref::<i32>() {
        Some(*v as f64)
    } else if let Some(v) = value.downcast_ref::<i64>() {
        Some(*v as f64)
    } else if let Some(v) = value.downcast_ref::<u32>() {
        Some(*v as f64)
    } else if let Some(v) = value.downcast_ref::<u64>() {
        Some(*v as f64)
    } else if let Some(v) = value.downcast_ref::<f32>() {
        Some(*v as f64)
    } else if let Some(v) = value.downcast_ref::<f64>() {
        Some(*v)
    } else {
        None
    }
}
