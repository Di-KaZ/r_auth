use auth_api::{create_user, models::NewUserData};
use dotenv::dotenv;
use lapin::{
    options::{BasicPublishOptions, QueueDeclareOptions},
    types::FieldTable,
    BasicProperties, Channel, Connection, ConnectionProperties,
};
use rocket::{form::Form, response::content, State};
use rocket_sync_db_pools::database;

extern crate dotenv;
#[macro_use]
extern crate rocket;

#[database("auth")]
struct AuthDatabase(rocket_sync_db_pools::diesel::PgConnection);

#[post("/register", data = "<user>")]
async fn register(
    conn: AuthDatabase,
    user_register_queue: &State<Channel>,
    user: Form<NewUserData>,
) -> content::RawJson<String> {
    let new_user = conn.run(|c| create_user(c, &user.into_inner())).await;

    match serde_json::to_string(&new_user) {
        Ok(json) => {
            user_register_queue
                .basic_publish(
                    "",
                    "register",
                    BasicPublishOptions::default(),
                    json.as_bytes(),
                    BasicProperties::default(),
                )
                .await
                .unwrap()
                .await
                .unwrap();
            content::RawJson(json)
        }
        Err(e) => content::RawJson(format!("{{\"error\": \"{}\"}}", e.to_string())),
    }
}

#[launch]
async fn rocket() -> _ {
    dotenv().ok();
    let amqp_url = std::env::var("AMQP_URL");

    let amqp_conn = Connection::connect(&amqp_url.unwrap(), ConnectionProperties::default())
        .await
        // rust error management be like
        .unwrap();

    let user_channel = amqp_conn.create_channel().await.unwrap();

    let _user_register_queue = user_channel
        .queue_declare(
            "register",
            QueueDeclareOptions::default(),
            FieldTable::default(),
        )
        .await
        .unwrap();

    rocket::build()
        .manage(user_channel)
        .attach(AuthDatabase::fairing())
        .mount("/", routes![register])
}
