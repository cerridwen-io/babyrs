use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenvy::dotenv;
use lazy_static::lazy_static;
use std::env;

pub mod models;
pub mod schema;

lazy_static! {
    static ref DB_KEY: &'static str = "DATABASE_URL";
}

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url: String = env::var(*DB_KEY).expect("Database URL must be set");

    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
