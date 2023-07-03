use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenvy::dotenv;
use lazy_static::lazy_static;
use log::debug;
use std::env;

pub mod models;
pub mod schema;

lazy_static! {
    static ref DB_KEY: &'static str = "DATABASE_URL";
}

fn get_database_url() -> String {
    dotenv().ok();

    env::var(*DB_KEY).expect("DATABASE_URL must be set")
}

pub fn establish_connection() -> SqliteConnection {
    let database_url: String = get_database_url();

    debug!("Connecting to {}", database_url);

    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_database_url() {
        std::env::set_var(*DB_KEY, "sqlite://test.db");
        assert_eq!(get_database_url(), "sqlite://test.db");
    }
}
