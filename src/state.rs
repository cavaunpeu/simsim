use std::collections::HashMap;

pub trait BaseState {
    fn get_serializable_record(&self) -> HashMap<&str, f64>;
}
