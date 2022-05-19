use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct DbValue<T> {
    pub id: String,
    pub lifetime: u64,
    pub value: T,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ValueType {
    #[serde(rename = "str")]
    String,
    #[serde(rename = "i32")]
    Integer32,
    #[serde(rename = "i64")]
    Integer64,
    #[serde(rename = "i128")]
    Integer128,
    #[serde(rename = "f32")]
    Float32,
    #[serde(rename = "f64")]
    Float64,
}
