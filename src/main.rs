extern crate rocket;
use rocket::fs::FileServer;
use rocket::response::Debug;
use rocket::{launch, routes};
use rocket_dyn_templates::Template;
//https://rocket.rs/guide/v0.5/requests/

pub mod auth;
pub mod claims;
pub mod db;
pub mod models;
pub mod schema;
pub mod venues;

pub static STATIC_FILES_DIR: &str = "www/static";

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![auth::login])
        .mount(
            "/venues",
            routes![venues::add, venues::delete, venues::get, venues::list],
        )
        .mount("/public", FileServer::from(STATIC_FILES_DIR))
        .attach(Template::fairing())
        .attach(db::TourDB::fairing())
}
