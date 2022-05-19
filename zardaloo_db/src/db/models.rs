use crate::DbValue;

pub struct Db<T> {
    pub values: Vec<DbValue<T>>,
}