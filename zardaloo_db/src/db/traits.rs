use crate::{DbValue, ValueType};

pub trait DbTrait<T> {
    fn set(&mut self, value: T, lifetime: u64, value_type: ValueType) -> Result<DbValue<T>, String>;
    fn get(&self, id: String) -> Result<DbValue<T>, String>;
    fn delete(&mut self, id: String) -> Result<DbValue<T>, String>;
}