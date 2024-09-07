// Generated by diesel_ext

#![allow(unused)]
#![allow(clippy::all)]

use crate::schema::*;
use rocket::serde::*;

use uuid::Uuid;
use diesel::prelude::*;
#[derive(Insertable, Queryable, Debug, Identifiable, Serialize, Deserialize)]
#[diesel(primary_key(venueid))]
#[diesel(table_name = venue)]
pub struct Venue {
    pub venueid: Uuid,
    pub name: Option<String>,
    pub contactname: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub postalcode: Option<String>,
    pub country: Option<String>,
    pub phone: Option<String>,
    pub latitude: Option<String>,
    pub longitude: Option<String>,
}
