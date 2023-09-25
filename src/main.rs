#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket_sync_db_pools;

mod auth;
mod models;
mod repository;
mod routes;
mod schema;

use rocket::serde::json::{json, Value};
use routes::*;

#[database("sqlite")]
pub struct DbConn(diesel::SqliteConnection);

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount(
            "/",
            routes![show_user, get_users, delete_user, update_user, create_user],
        )
        .register("/", catchers![unauthorize_handler, notfound_handler])
        .attach(DbConn::fairing())
        .launch()
        .await;
}

#[catch(401)]
fn unauthorize_handler() -> Value {
    json!({"data": "you don't have access"})
}

#[catch(404)]
fn notfound_handler() -> Value {
    json!({"data": "NOT FOUND"})
}
