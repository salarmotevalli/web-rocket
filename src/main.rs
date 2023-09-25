#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket_sync_db_pools;

mod auth;
mod models;
mod repository;
mod schema;

use auth::BasicAuth;
use models::NewUser;
use repository::UserRepository;
use rocket::response::status;
use rocket::serde::json::{json, Json, Value};

#[database("sqlite")]
struct DbConn(diesel::SqliteConnection);

#[get("/users")]
async fn get_users(_auth: BasicAuth, db: DbConn) -> Value {
    db.run(|c| {
        let result = UserRepository::find_multi(c, 100).expect("Failed to read users table");

        json!(result)
    })
    .await
}

#[get("/users/<id>")]
async fn show_user(db: DbConn, id: i32) -> Value {
    db.run(move |c| {
        let result = UserRepository::find(c, id).expect("Failed to read users table");
        json!(result)
    })
    .await
}

#[post("/users", format = "json", data = "<new_user>")]
async fn create_user(_auth: BasicAuth, db: DbConn, new_user: Json<NewUser>) -> Value {
    db.run(|c| {
        let result = UserRepository::create(c, new_user.into_inner())
            .expect("cannot write into users table");
        json!(result)
    })
    .await
}

#[put("/users/<id>", format = "json", data = "<new_user>")]
async fn update_user(id: i32, _auth: BasicAuth, db: DbConn, new_user: Json<NewUser>) -> Value {
    db.run(move |c| {
        let result = UserRepository::save(c, id, new_user.into_inner())
            .expect("cannot write into users table");
        json!(result)
    })
    .await
}

#[delete("/users/<id>")]
async fn delete_user(id: i32, _auth: BasicAuth, db: DbConn) -> status::NoContent {
    db.run(move |c| {
        UserRepository::delete(c, id).expect("cannot delete record");
        status::NoContent
    })
    .await
}

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
    json!({"data": "you don't have any access bitch"})
}

#[catch(404)]
fn notfound_handler() -> Value {
    json!({"data": "what the fuck you want bitch"})
}
