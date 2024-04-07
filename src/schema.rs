use diesel::prelude::*;

table! {
    users(id) {
        id -> Int4,
        name -> Varchar,
        email -> Varchar,
    }
}