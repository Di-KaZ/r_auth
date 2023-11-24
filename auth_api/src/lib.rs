use crate::models::{NewUser, User};
use diesel::{PgConnection, RunQueryDsl, SelectableHelper};
use models::NewUserData;

pub mod models;
pub mod schema;

pub fn create_user(conn: &mut PgConnection, user: &NewUserData) -> User {
    use crate::schema::users;

    diesel::insert_into(users::table)
        .values(NewUser {
            username: &user.username.as_str(),
            email: &user.email.as_str(),
            verified: false,
        })
        .returning(User::as_returning())
        .get_result(conn)
        .expect("Error saving new user")
}
