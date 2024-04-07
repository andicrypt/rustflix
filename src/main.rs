mod args;
mod db;
mod models;
mod schema;

use args::RustflixArgs;
use clap::Parser;
use schema::users;

use crate::args::EntityType;
use crate::db::setablish_connection;
use diesel::prelude::*;

fn main() {
    let connection = &mut setablish_connection();
    let args = RustflixArgs::parse();

    match &args.entity_type {
        EntityType::User(user_command) => user_handler(connection,&user_command),
        _ => default()
        // VideoSubcommand => unimplemented!(),
        // ViewSubcommand => unimplemented!(),
    }
}

fn user_handler(conn: &mut PgConnection, user_command: &args::UserCommand) {
    match &user_command.command {
        args::UserSubcommand::Create(create_user) => {
            create_new_user(conn, create_user.name.clone(), create_user.email.clone())
        }
        args::UserSubcommand::Delete(delete_user) => {
            delete_existing_user(conn, delete_user.id.clone());
        },
        args::UserSubcommand::Show => show_users(conn),
        _ => unimplemented!(),
    }
}

fn default() {
    println!("default!");
}

#[derive(Insertable)]
#[diesel(table_name = users)]
struct NewUser {
    name: String,
    email: String,
}

fn create_new_user(conn: &mut PgConnection, name: String, email: String) {
    let new_user = NewUser { name, email };
    diesel::insert_into(users::table)
        .values(&new_user)
        .execute(conn)
        .expect("Error create new user");
}

fn delete_existing_user(conn: &mut PgConnection, id: i32) {
    diesel::delete(users::table.find(id))
        .execute(conn)
        .expect("Error deleting user");
}

fn show_users(conn: &mut PgConnection) {
    let users = users::table
        .load::<models::User>(conn)
        .expect("Error loading users");
    for user in users {
        println!("{:?}", user);
    }
}
