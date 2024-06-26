use diesel::prelude::*;

#[derive(Debug, Queryable)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
}