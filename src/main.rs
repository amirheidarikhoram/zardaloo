pub mod lib;
pub mod controllers;
pub mod statics;

use actix_web::{App, HttpServer};
use dotenv::dotenv;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    dotenv().ok();
    statics::init();


    HttpServer::new(|| App::new())
        .bind(("127.0.0.1", *statics::PORT))?
        .run()
        .await
}
