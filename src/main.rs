pub mod controllers;
pub mod lib;
pub mod statics;

use actix_web::{web::Data, App, HttpServer};
use controllers::{data, user};
use dotenv::dotenv;
use std::sync::Mutex;
use zardaloo_db::DbSet;

#[allow(dead_code)]
struct AppState {
    db: Mutex<DbSet>,
}

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    dotenv().ok();
    statics::init();

    let state = Data::new(AppState {
        db: Mutex::new(DbSet::new()),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .configure(user::init_routes)
            .configure(data::init_routes)
    })
    .bind(("127.0.0.1", *statics::PORT))?
    .run()
    .await
}
