#[macro_use]
extern crate rocket;

mod tictactoe;
mod errors;


#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount(
            "/",
            routes![
                tictactoe::routes::index,
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
