use crate::schema::users;
use diesel::prelude::*;

use rocket::FromForm;
use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub verified: bool,
}

#[derive(Insertable, Debug, PartialEq, Serialize, Deserialize, Clone)]
#[diesel(table_name = users)]
pub struct NewUser<'r> {
    pub username: &'r str,
    pub email: &'r str,
    pub verified: bool,
}

#[derive(FromForm)]
pub struct NewUserData {
    pub username: String,
    pub email: String,
}
