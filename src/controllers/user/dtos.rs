pub mod request {
    use serde::Deserialize;

    #[derive(Deserialize)]
    pub struct LoginRequestDTO {
        pub username: String,
        pub password: String,
    }
}

pub mod response {
    use serde::Serialize;

    #[derive(Serialize)]
    pub struct LoginResponseDTO {
        pub token: String,
    }
}
