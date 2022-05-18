use lazy_static::lazy_static;
use std::env;

lazy_static! {
    pub static ref USERNAME: String = {
        let username = env::var("USERNAME").expect("Username must be provided");
        username
    };
    pub static ref PASSWORD: String = {
        let password = env::var("PASSWORD").expect("Password must be provided");
        password
    };
    pub static ref PORT: u16 = {
        let port: u16 = env::var("APPLICATION_PORT")
            .expect("PORT must be provided")
            .parse::<u16>()
            .expect("PORT must be u16");
        port
    };
}

pub fn init() {
    lazy_static::initialize(&USERNAME);
    lazy_static::initialize(&PASSWORD);
    lazy_static::initialize(&PORT);
}
