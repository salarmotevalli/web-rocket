use crate::models::NewUser;
use crate::schema::users;
use diesel::prelude::*;
use diesel::{QueryDsl, QueryResult, SqliteConnection};

use super::models::User;

pub struct UserRepository;

impl UserRepository {
    pub fn find(c: &mut SqliteConnection, id: i32) -> QueryResult<User> {
        users::table.find(id).get_result::<User>(c)
    }

    pub fn find_multi(c: &mut SqliteConnection, limit: i64) -> QueryResult<Vec<User>> {
        users::table.limit(limit).load::<User>(c)
    }

    pub fn create(c: &mut SqliteConnection, new_user: NewUser) -> QueryResult<usize> {
        diesel::insert_into(users::table)
            .values(new_user)
            .execute(c)
    }

    pub fn save(c: &mut SqliteConnection, id: i32, new_user: NewUser) -> QueryResult<usize> {
        diesel::update(users::dsl::users.find(id))
            .set((
                users::email.eq(new_user.email.to_owned()),
                users::name.eq(new_user.name.to_owned()),
            ))
            .execute(c)
    }

    pub fn delete(c: &mut SqliteConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(users::table.find(id)).execute(c)
    }
}
