use crate::Db;

pub struct DbSet {
    pub string_db: Db<String>,
    pub signed_32_integer_db: Db<i32>,
    pub signed_64_integer_db: Db<i64>,
    pub signed_128_integer_db: Db<i128>,
    pub float_32_db: Db<f32>,
    pub float_64_db: Db<f64>,
}