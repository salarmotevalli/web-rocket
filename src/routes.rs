use super::auth::BasicAuth;
use super::models::NewUser;
use super::repository::UserRepository;
use super::DbConn;
use rocket::response::status;
use rocket::serde::json::{json, Json, Value};

#[get("/users")]
pub async fn get_users(_auth: BasicAuth, db: DbConn) -> Value {
    db.run(|c| {
        let result = UserRepository::find_multi(c, 100).expect("Failed to read users table");

        json!(result)
    })
    .await
}

#[get("/users/<id>")]
pub async fn show_user(db: DbConn, id: i32) -> Value {
    db.run(move |c| {
        let result = UserRepository::find(c, id).expect("Failed to read users table");
        json!(result)
    })
    .await
}

#[post("/users", format = "json", data = "<new_user>")]
pub async fn create_user(_auth: BasicAuth, db: DbConn, new_user: Json<NewUser>) -> Value {
    db.run(|c| {
        let result = UserRepository::create(c, new_user.into_inner())
            .expect("cannot write into users table");
        json!(result)
    })
    .await
}

#[put("/users/<id>", format = "json", data = "<new_user>")]
pub async fn update_user(id: i32, _auth: BasicAuth, db: DbConn, new_user: Json<NewUser>) -> Value {
    db.run(move |c| {
        let result = UserRepository::save(c, id, new_user.into_inner())
            .expect("cannot write into users table");
        json!(result)
    })
    .await
}

#[delete("/users/<id>")]
pub async fn delete_user(id: i32, _auth: BasicAuth, db: DbConn) -> status::NoContent {
    db.run(move |c| {
        UserRepository::delete(c, id).expect("cannot delete record");
        status::NoContent
    })
    .await
}
