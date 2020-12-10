#![feature(decl_macro)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

mod routes;
use routes::*;

fn main() {
    rocket::ignite()
        .mount("/", routes![ast])
        .register(catchers![not_found])
        .launch();
}
