use super::Db;
use super::DbTrait;
use crate::DbValue;
use crate::ValueType;

impl<T> Db<T> {
    pub fn new() -> Self {
        Self { values: Vec::new() }
    }
}

impl<T> DbTrait<T> for Db<T>
where
    T: Clone,
{
    fn set(&mut self, value: T, lifetime: u64, value_type: ValueType) -> Result<DbValue<T>, String> {
        // let v_type;
        // match  value_type {
        //     Some(_v_type) => v_type = _v_type, 
        //     None => v_type = ValueType::String
        // }
        let db_value = DbValue::new(value, lifetime, value_type);
        self.values.push(db_value.clone());

        Ok(db_value)
    }

    fn get(&self, id: String) -> Result<DbValue<T>, String> {
        match self.values.iter().find(|&db_value| db_value.id == id) {
            Some(val) => Ok(val.clone()),
            None => Err("Couldn't find value in db".to_string()),
        }
    }

    fn delete(&mut self, id: String) -> Result<DbValue<T>, String> {
        match self.values.iter().position(|db_value| db_value.id == id) {
            Some(position) => {
                let deleted = self.values.remove(position);
                Ok(deleted)
            }
            None => Err("Couldn't find value".to_string()),
        }
    }
}
