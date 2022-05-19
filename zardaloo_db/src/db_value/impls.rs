use crate::ValueType;
use core::fmt;
use uuid::Uuid;

use super::DbValue;

impl<T> DbValue<T> {
    pub fn new(value: T, lifetime: u64, value_type: ValueType) -> Self {
        Self {
            id: format!("{}#{}",  Uuid::new_v4(), value_type),
            value,
            lifetime,
        }
    }
}

impl fmt::Display for ValueType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValueType::String => write!(f, "str"),
            ValueType::Integer32 => write!(f, "i32"),
            ValueType::Integer64 => write!(f, "i64"),
            ValueType::Integer128 => write!(f, "i128"),
            ValueType::Float32 => write!(f, "f32"),
            ValueType::Float64 => write!(f, "f64"),
        }
    }
}
