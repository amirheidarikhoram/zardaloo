use crate::DbValue;

pub trait DbSetOperationsTrait<T> {
    fn set(&mut self, value: T, lifetime: u64) -> Result<DbValue<T>, String>;
    fn get(&self, id: String) -> Result<DbValue<T>, String>;
    fn delete(&mut self, id: String) -> Result<DbValue<T>, String>;
}
