use rocket_sync_db_pools::diesel;
use rocket_sync_db_pools::database;

#[database("postgres")]
pub struct TourDB(diesel::PgConnection);
