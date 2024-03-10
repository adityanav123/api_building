extern crate diesel;

extern crate dotenv;
extern crate r2d2;
extern crate rocket;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use dotenv::dotenv;
use rocket::{catchers, fairing::AdHoc, routes, Build};
use routes::*;
use std::env;

mod db;
mod models;
mod routes;
mod schema;
mod static_files;

fn rocket() -> rocket::Rocket<Build> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("set DATABASE_URL");
    let pool = db::init_pool(database_url);
    rocket::build()
        .attach(AdHoc::on_ignite("database pool", |rocket| async {
            rocket.manage(pool)
        }))
        .mount(
            "/api/v1/",
            routes![index, new, show, delete, author_books, update],
        )
        .mount("/", routes![static_files::index, static_files::all])
        .register("/", catchers![not_found])
}

#[rocket::main]
async fn main() {
    if let Err(e) = rocket().launch().await {
        println!("Failed to launch rocket = {}", e);
    }
}
