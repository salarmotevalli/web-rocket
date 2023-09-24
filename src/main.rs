#[macro_use]
extern crate rocket;

use rocket::serde::json::{json, Value};

mod auth;

#[get("/secrete")]
fn secrete(_auth: auth::BasicAuth) -> Value {
    json!({"data": "secrete data"})
}

#[get("/")]
fn hello() -> Value {
    json!(vec!["fuck"])
}

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount("/", routes![hello, secrete])
        .register("/", catchers![unauthorize_handler, notfound_handler])
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
