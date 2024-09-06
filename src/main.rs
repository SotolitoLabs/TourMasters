#[macro_use] extern crate rocket;
extern crate diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use rocket::response::{status::Created, Debug};
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::{get, post, launch, routes};
use rocket_dyn_templates::{context, Template};
use rocket::fs::FileServer;
use std::env;
use uuid::Uuid;
//https://rocket.rs/guide/v0.5/requests/
use crate::models::*;

pub mod models;
pub mod schema;

pub static STATIC_FILES_DIR: &str = "www/static";

pub fn establish_connection_pg() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

type Result<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;

#[post("/add", format = "json", data = "<arg_venue>")]
pub fn create_venue(mut arg_venue: Json<Venue>) -> Result<Created<Json<Venue>>> {
    use self::schema::venue::dsl::*;
    use models::Venue;
    let connection = &mut establish_connection_pg();
    let mut other_venue: models::Venue = arg_venue.into_inner();
    other_venue.venueid = Uuid::new_v4();
    /*let new_venue = Venue {
        venueid: Uuid::new_v4(),
        name: Option<String>,
        contactname: Option<String>,
        address: Option<String>,
        city: Option<String>,
        postalcode: Option<String>,
        country: Option<String>,
        phone: Option<String>,
        latitude: Option<i32>,
        longitude: Option<i32>,
    }*/

    diesel::insert_into(self::schema::venue::dsl::venue)
        .values(&other_venue)
        .execute(connection)
        .expect("Error saving new venue");
    Ok(Created::new("/").body(Json(other_venue)))
}

#[get("/")]
pub fn list_venues() -> Template {
    use self::models::Venue;
    let connection = &mut establish_connection_pg();
    let results = self::schema::venue::dsl::venue
        .load::<Venue>(connection)
        .expect("Error loading venues");
    Template::render("venues", context! {venues: &results, count: results.len()})
}

/*
#[get("/<id>")]
pub fn get_venue(id: Uuid) -> Venue {
    use self::models::Venue;
    let connection = &mut establish_connection_pg();
    let results = self::schema::venue::dsl::venue
        .load::<Venue>(connection)
        .expect("Error loading venues");
}
*/


#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/venues", routes![create_venue])
        .mount("/venues", routes![list_venues])
        .mount("/public", FileServer::from(STATIC_FILES_DIR))
        .attach(Template::fairing())
}
