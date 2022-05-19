use super::DbSet;
use super::DbSetOperationsTrait;
use crate::Db;
use crate::DbTrait;
use crate::ValueType;

impl DbSet {
    pub fn new() -> Self {
        Self {
            string_db: Db::new(),
            signed_32_integer_db: Db::new(),
            signed_64_integer_db: Db::new(),
            signed_128_integer_db: Db::new(),
            float_32_db: Db::new(),
            float_64_db: Db::new(),
        }
    }
}

impl DbSetOperationsTrait<String> for DbSet {
    fn set(&mut self, value: String, lifetime: u64) -> Result<crate::DbValue<String>, String> {
        self.string_db.set(value, lifetime, ValueType::String)
    }

    fn get(&self, id: String) -> Result<crate::DbValue<String>, String> {
        self.string_db.get(id)
    }

    fn delete(&mut self, id: String) -> Result<crate::DbValue<String>, String> {
        self.string_db.delete(id)
    }
}

impl DbSetOperationsTrait<i32> for DbSet {
    fn set(&mut self, value: i32, lifetime: u64) -> Result<crate::DbValue<i32>, String> {
        self.signed_32_integer_db
            .set(value, lifetime, ValueType::Integer32)
    }

    fn get(&self, id: String) -> Result<crate::DbValue<i32>, String> {
        self.signed_32_integer_db.get(id)
    }

    fn delete(&mut self, id: String) -> Result<crate::DbValue<i32>, String> {
        self.signed_32_integer_db.delete(id)
    }
}

impl DbSetOperationsTrait<i64> for DbSet {
    fn set(&mut self, value: i64, lifetime: u64) -> Result<crate::DbValue<i64>, String> {
        self.signed_64_integer_db
            .set(value, lifetime, ValueType::Integer64)
    }

    fn get(&self, id: String) -> Result<crate::DbValue<i64>, String> {
        self.signed_64_integer_db.get(id)
    }

    fn delete(&mut self, id: String) -> Result<crate::DbValue<i64>, String> {
        self.signed_64_integer_db.delete(id)
    }
}

impl DbSetOperationsTrait<i128> for DbSet {
    fn set(&mut self, value: i128, lifetime: u64) -> Result<crate::DbValue<i128>, String> {
        self.signed_128_integer_db
            .set(value, lifetime, ValueType::Integer128)
    }

    fn get(&self, id: String) -> Result<crate::DbValue<i128>, String> {
        self.signed_128_integer_db.get(id)
    }

    fn delete(&mut self, id: String) -> Result<crate::DbValue<i128>, String> {
        self.signed_128_integer_db.delete(id)
    }
}

impl DbSetOperationsTrait<f32> for DbSet {
    fn set(&mut self, value: f32, lifetime: u64) -> Result<crate::DbValue<f32>, String> {
        self.float_32_db.set(value, lifetime, ValueType::Float32)
    }

    fn get(&self, id: String) -> Result<crate::DbValue<f32>, String> {
        self.float_32_db.get(id)
    }

    fn delete(&mut self, id: String) -> Result<crate::DbValue<f32>, String> {
        self.float_32_db.delete(id)
    }
}

impl DbSetOperationsTrait<f64> for DbSet {
    fn set(&mut self, value: f64, lifetime: u64) -> Result<crate::DbValue<f64>, String> {
        self.float_64_db.set(value, lifetime, ValueType::Float64)
    }

    fn get(&self, id: String) -> Result<crate::DbValue<f64>, String> {
        self.float_64_db.get(id)
    }

    fn delete(&mut self, id: String) -> Result<crate::DbValue<f64>, String> {
        self.float_64_db.delete(id)
    }
}
