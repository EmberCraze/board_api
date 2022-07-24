#[macro_use]
extern crate rocket;

use sea_orm::{entity::*, query::*};
use sea_orm_rocket::{Connection, Database};

mod setup_sea_orm;
use setup_sea_orm::Db;
mod tictactoe;
mod errors;


#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Db::init())
        .mount(
            "/",
            routes![
                tictactoe::routes::index,
                tictactoe::routes::board,
            ],
        )
        .register(
            "/",
            catchers![
                // errors::default_error,
                // errors::not_found_error,
                // errors::internal_server_error,
            ]
        )
                
}
