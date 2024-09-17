use rocket_sync_db_pools::diesel;
use rocket_sync_db_pools::database;

#[database("postgres")]
pub struct TourDB(diesel::PgConnection);

/*extern crate diesel;
use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use rocket::fairing::AdHoc;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
/*use uuid::Uuid;
//https://rocket.rs/guide/v0.5/requests/
use self::models::Venue;
use self::schema::venue::dsl::*;
*/

pub type DBPool = r2d2::Pool<ConnectionManager<PgConnection>>;

/// Temporal connection routine
pub fn establish_connection_pg() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))

}

/// Database Connecttion pool
pub fn init_db_pool() -> DBPool {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    r2d2::Pool::builder().build(manager).expect("Error creating database pool")
}

/// Attaches the database pool to the rocket instance
pub fn attach_db_pool() -> AdHoc {
    AdHoc::try_on_ignite("Initializing database pool", |rocket| async {
        let pool = init_db_pool();
        Ok(rocket.manage(pool))
    })
}*/
