pub mod request {
    use serde::{Deserialize};
    use zardaloo_db::ValueType;

    #[derive(Deserialize)]
    pub struct SetRequestDTO {
        pub lifetime: u64,
        pub value: String,
        pub value_type: Option<ValueType>
    }

    #[derive(Deserialize)]
    pub struct GetRequestDTO {
        pub id: String,
    }

    #[derive(Deserialize)]
    pub struct DeleteRequestDTO {
        pub id: String,
    }
}

pub mod response {
    use serde::Serialize;

    #[derive(Serialize)]
    pub struct DataResponseDTO<T: Serialize> {
        pub value: T,
        pub lifetime: u64,
        pub id: String
    }
}

pub use request::*;
pub use response::*;