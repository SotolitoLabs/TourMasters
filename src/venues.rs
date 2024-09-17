use diesel::prelude::*;
use rocket::response::{status::Created, Debug};
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::serde::uuid::Uuid;
use rocket::{get, post, delete};
use rocket_dyn_templates::{context, Template};
//use uuid::Uuid;
//https://rocket.rs/guide/v0.5/requests/
use crate::db::*;
use crate::models::Venue;
use crate::schema::venue::dsl::*;
use rocket_sync_db_pools::diesel;

type Result<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;

/// Creates a venue
#[post("/add", format = "json", data = "<arg_venue>")]
//pub async fn add(mut arg_venue: Json<Venue>, mut tdb: TourDB) -> Result<Created<Json<Venue>>> {
pub async fn add(mut arg_venue: Json<Venue>, mut tdb: TourDB) -> Result<Created<Json<Uuid>>> {
    let mut new_venue: Venue = arg_venue.into_inner();
    new_venue.id = Uuid::new_v4();
    let ret_id = new_venue.id.clone();
    let venue_id = tdb.run(move |conn| {
        diesel::insert_into(crate::schema::venue::dsl::venue)
            .values(&new_venue)
            .execute(conn)
            .expect("Error saving new venue");
    }).await;

    Ok(Created::new("/").body(Json(ret_id)))
}

//https://api.rocket.rs/v0.5/rocket_sync_db_pools/

/// Show the list of venues in HTML
#[get("/")]
pub async fn list(mut tdb: TourDB) -> Template {
    let mut results =
    tdb.run(move |connection| 
        crate::schema::venue::dsl::venue
            .load::<Venue>(connection)
            .expect("Error loading venues")
    ).await;
    Template::render("venues", context! {venues: &results, count: results.len()})
}
/*
/// Get a venue and returns it as a JSON object
#[get("/<venueid>")]
pub fn get(venueid: Uuid) -> Json<Vec<Venue>> {
    let connection = &mut establish_connection_pg();
    let results = crate::schema::venue::dsl::venue
        .filter(id.eq(venueid))
        .load::<Venue>(connection)
        .expect("Error loading venues");
    Json(results)
}

/// Remove a venue
#[delete("/<venueid>")]
pub fn delete(venueid: Uuid) -> Json<Vec<Venue>> {
    let connection = &mut establish_connection_pg();
    let results = crate::schema::venue::dsl::venue
        .filter(id.eq(venueid))
        .load::<Venue>(connection)
        .expect("Error loading venues");
    Json(results)
}
*/

