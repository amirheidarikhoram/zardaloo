use lazy_static::lazy_static;
use std::env;

lazy_static! {
    pub static ref USERNAME: String = {
        let username = env::var("USERNAMEX").expect("Username must be provided");
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
    pub static ref SECRET: String = {
        let secret = env::var("TOKENSECRET")
            .expect("SECRET must be provided");
        secret
    };
}

pub fn init() {
    lazy_static::initialize(&USERNAME);
    lazy_static::initialize(&PASSWORD);
    lazy_static::initialize(&PORT);
    lazy_static::initialize(&SECRET);
}