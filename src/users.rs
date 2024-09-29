use diesel::prelude::*;
use rocket::response::{status::Created, Debug};
use rocket::serde::json::Json;
use rocket::serde::uuid::Uuid;
use rocket::{get, post, delete};
use rocket_dyn_templates::{context, Template};
use rocket_sync_db_pools::diesel;
use rocket::response::status::NotFound;
use crate::db::*;
use crate::claims::Claims;
use crate::models::Users;
use crate::schema::venue::dsl::*;

type Result<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;

/// Creates a venue
#[post("/add", format = "json", data = "<arg_user>")]
pub async fn add(arg_venue: Json<Users>, user: Claims, tdb: TourDB) ->
Result<Created<Json<Uuid>>> {
    let mut new_user: Users = arg_venue.into_inner();
    new_user.id = Uuid::new_v4();
    let ret_id = new_user.id.clone();
    let user_id = tdb.run(move |conn| {
        diesel::insert_into(crate::schema::users::dsl::users)
            .values(&new_users)
            .execute(conn)
            .expect("Error saving new users");
    }).await;

    Ok(Created::new("/").body(Json(ret_id)))
}

//https://api.rocket.rs/v0.5/rocket_sync_db_pools/

/// Show the list of venues in HTML
#[get("/")]
pub async fn list(tdb: TourDB) -> Template {
    let results =
    tdb.run(move |connection| 
        crate::schema::venue::dsl::venue
            .load::<Users>(connection)
            .expect("Error loading user")
    ).await;
    Template::render("users", context! {users: &results, count: results.len()})
}

/// Get a venue and returns it as a JSON object
#[get("/<venueid>")]
pub async fn get(userid: Uuid, tdb: TourDB) ->
Result<Json<Vec<Users>>, NotFound<String>> {
    let results = tdb.run(move |connection|
        crate::schema::users::dsl::users
            .filter(id.eq(usersid))
            .load::<Users>(connection)
        .expect("Error loading users")
    ).await;
    if results.len() > 0 {
        Ok(Json(results))
    } else {
        Err(NotFound(format!("Could not find user: {}", userid)))
    }
}

/// Remove a venue
#[delete("/<userid>")]
pub async fn delete(userid: Uuid, user: Claims, tdb: TourDB) ->
Result<Json<String>, NotFound<String>> {
    let results = tdb.run(move |connection|
        diesel::delete(
            crate::schema::users::dsl::users
                .filter(id.eq(userid)))
            .execute(connection)
    ).await;
    if results.unwrap() == 1 {
        Ok(Json(format!("{} deleted", userid)))
    } else {
        Err(NotFound(format!("Could not find user: {}", userid)))
    }
}

