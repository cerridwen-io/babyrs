/// Module handling database operations for baby events.
///
/// This module provides functionalities for CRUD operations as well as processing CSV files.
use csv::Reader;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenvy::dotenv;
use lazy_static::lazy_static;
use log::{debug, info};
use models::{BabyEvent, NewBabyEvent};
use std::{env, error::Error, fs::File};

pub mod models;
pub mod schema;

lazy_static! {
    static ref DB_KEY: &'static str = "DATABASE_URL";
}

/// Fetches the database URL from environment variables.
///
/// # Returns
///
/// A string containing the database URL.
fn get_database_url() -> String {
    dotenv().ok();

    env::var(*DB_KEY).expect("DATABASE_URL must be set")
}

/// Establishes a connection to the SQLite database.
///
/// # Returns
///
/// An established SQLiteConnection object.
pub fn establish_connection() -> SqliteConnection {
    let database_url: String = get_database_url();

    debug!("Connecting to {}", database_url);

    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

/// Creates a new baby event.
///
/// # Arguments
///
/// Various optional arguments for different kinds of baby events.
///
/// # Returns
///
/// A NewBabyEvent object.
pub fn create_event(
    urine: Option<bool>,
    stool: Option<bool>,
    skin2skin: Option<u16>,
    breastfeed: Option<u16>,
    breastmilk: Option<u16>,
    formula: Option<u16>,
    pump: Option<u16>,
) -> NewBabyEvent {
    debug!("Creating event - urine: {:?}, stool: {:?}, skin2skin: {:?}, breastfeed: {:?}, breastmilk: {:?}, formula: {:?}, pump: {:?}",
        &urine, &stool, &skin2skin, &breastfeed, &breastmilk, &formula, &pump);

    NewBabyEvent {
        dt: chrono::Local::now().naive_local(),
        urine: urine.unwrap_or(false),
        stool: stool.unwrap_or(false),
        skin2skin: i32::from(skin2skin.unwrap_or(0)),
        breastfeed: i32::from(breastfeed.unwrap_or(0)),
        breastmilk: i32::from(breastmilk.unwrap_or(0)),
        formula: i32::from(formula.unwrap_or(0)),
        pump: i32::from(pump.unwrap_or(0)),
    }
}

/// Writes a new baby event into the database.
///
/// # Arguments
///
/// - `connection`: The database connection.
/// - `new_event`: The baby event to write.
///
/// # Returns
///
/// The number of rows inserted.
pub fn write_event(connection: &mut SqliteConnection, new_event: NewBabyEvent) -> usize {
    debug!("Writing event: {:?}", &new_event);

    diesel::insert_or_ignore_into(schema::events::table)
        .values(&new_event)
        .execute(connection)
        .expect("Error saving new event")
}

/// Reads baby events from the database.
///
/// # Arguments
///
/// - `connection`: The database connection.
///
/// # Returns
///
/// A vector of BabyEvent objects.
pub fn read_events(connection: &mut SqliteConnection) -> Vec<BabyEvent> {
    use schema::events::dsl::*;

    info!("Reading events");

    let results: Vec<BabyEvent> = events
        .select(BabyEvent::as_select())
        .load(connection)
        .expect("Error loading events");

    debug!("Read events: {:?}", &results);

    results
}

/// Updates an existing baby event in the database.
///
/// # Arguments
///
/// - `connection`: The database connection.
/// - `event`: The baby event to update.
///
/// # Returns
///
/// The number of rows updated.
pub fn update_event(connection: &mut SqliteConnection, event: BabyEvent) -> usize {
    use schema::events::dsl::*;

    debug!("Updating event: {:?}", &event);

    diesel::update(events.find(event.id))
        .set(&event)
        .execute(connection)
        .expect("Error updating event")
}

/// Deletes an existing baby event in the database.
///
/// # Arguments
///
/// - `connection`: The database connection.
/// - `event`: The baby event to delete.
///
/// # Returns
///
/// The number of rows deleted.
pub fn delete_event(connection: &mut SqliteConnection, event: BabyEvent) -> usize {
    use schema::events::dsl::*;

    debug!("Deleting event: {:?}", &event);

    diesel::delete(events.find(event.id))
        .execute(connection)
        .expect("Error deleting event")
}

/// Processes a CSV file and writes the baby events into the database.
///
/// # Arguments
///
/// - `connection`: The database connection.
/// - `file_path`: The path of the CSV file.
///
/// # Returns
///
/// Returns a `Result` indicating success or an error.
pub fn process_csv(
    connection: &mut SqliteConnection,
    file_path: &str,
) -> Result<(), Box<dyn Error>> {
    info!("Processing CSV file: {}", &file_path);

    let mut rdr: Reader<File> = Reader::from_path(file_path)?;

    for result in rdr.deserialize() {
        let record: NewBabyEvent = result?;

        debug!("Read record: {:?}", &record);

        write_event(connection, record);
    }

    info!("Processed CSV file: {}", &file_path);

    Ok(())
}

#[cfg(test)]
mod tests {
    /// Tests for the database module.
    use super::*;

    #[test]
    /// Test to ensure get_database_url returns the correct database URL.
    fn test_get_database_url() {
        std::env::set_var(*DB_KEY, "sqlite://test.db");
        assert_eq!(get_database_url(), "sqlite://test.db");
    }

    #[test]
    /// Test to ensure create_event creates the event correctly.
    fn test_create_event() {
        let new_event = create_event(
            Some(true),
            Some(true),
            Some(5),
            Some(10),
            Some(15),
            Some(20),
            Some(25),
        );

        assert_eq!(new_event.urine, true);
        assert_eq!(new_event.stool, true);
        assert_eq!(new_event.skin2skin, 5);
        assert_eq!(new_event.breastfeed, 10);
        assert_eq!(new_event.breastmilk, 15);
        assert_eq!(new_event.formula, 20);
        assert_eq!(new_event.pump, 25);

        let another_event = create_event(None, None, None, None, None, None, None);

        assert_eq!(another_event.urine, false);
        assert_eq!(another_event.stool, false);
        assert_eq!(another_event.skin2skin, 0);
        assert_eq!(another_event.breastfeed, 0);
        assert_eq!(another_event.breastmilk, 0);
        assert_eq!(another_event.formula, 0);
        assert_eq!(another_event.pump, 0);
    }
}
