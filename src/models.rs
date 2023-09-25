#[derive(serde::Serialize, Queryable, serde::Deserialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub created_at: String,
}

#[derive(serde::Serialize, serde::Deserialize, Insertable)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser {
    pub name: String,
    pub email: String,
}
