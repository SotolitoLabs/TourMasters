extern crate rocket;
use rocket::response::Debug;
use rocket::{launch, routes};
use rocket_dyn_templates::{Template};
use rocket::fs::FileServer;
//https://rocket.rs/guide/v0.5/requests/

pub mod claims;
pub mod auth;
pub mod db;
pub mod models;
pub mod schema;
pub mod venues;

pub static STATIC_FILES_DIR: &str = "www/static";

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![auth::login])
        .mount("/venues", routes![venues::add])
        .mount("/venues", routes![venues::list])
        .mount("/venues", routes![venues::get])
        .mount("/venues", routes![venues::delete])
        .mount("/public", FileServer::from(STATIC_FILES_DIR))
        .attach(Template::fairing())
        .attach(db::TourDB::fairing())
}
