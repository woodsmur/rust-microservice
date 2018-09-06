#![feature(
    plugin,
    custom_derive,
    const_fn,
    decl_macro,
    custom_attribute
)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use routes::*;
use std::env;

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

mod db;
mod models;
mod routes;
mod schema;
mod static_files;

fn rocket() -> rocket::Rocket {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("set DATABASE_URL");
    let pool = db::init_pool(database_url);

    rocket::ignite()
        .manage(pool)
        .mount(
            "/api/v1/",
            routes![index, new, show, delete, author, update],
        ).mount("/", routes![static_files::all, static_files::index])
        .catch(catchers![not_found])
}

fn main() {
    rocket().launch();
}
