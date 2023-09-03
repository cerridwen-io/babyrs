//! This module contains integration tests for baby-related event handling using the babyrs library.

use babyrs::{create_event, establish_connection, read_events, write_event};
use diesel::prelude::*;
use diesel::sqlite::Sqlite;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();
use babyrs::models::{BabyEvent, NewBabyEvent};
use std::error::Error;

/// Run pending database migrations.
///
/// # Arguments
///
/// * `connection`: Mutable reference to the database connection
///
/// # Errors
///
/// Returns an error if the migration fails.
fn run_migrations(
    connection: &mut impl MigrationHarness<Sqlite>,
) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    connection.run_pending_migrations(MIGRATIONS)?;

    Ok(())
}

/// Test database connection establishment.
///
/// This test checks if a SQLite database connection can be established.
#[test]
fn test_establish_connection() {
    std::env::set_var("DATABASE_URL", ":memory:");

    let connection: &mut SqliteConnection = &mut establish_connection();

    assert!(diesel::sql_query("SELECT 1").execute(connection).is_ok());
}

/// Test writing a new event to the database.
///
/// This test writes a single event to an empty database and then verifies its presence.
#[test]
fn test_write_event() {
    use babyrs::schema::events::dsl::*;

    std::env::set_var("DATABASE_URL", ":memory:");

    let connection: &mut SqliteConnection = &mut establish_connection();

    run_migrations(connection).expect("Error running migrations");

    let new_event: NewBabyEvent = create_event(
        Some(true),
        Some(true),
        Some(5),
        Some(10),
        Some(15),
        Some(20),
        Some(25),
    );

    assert_eq!(write_event(connection, new_event), 1);

    let results: Vec<BabyEvent> = events
        .load::<BabyEvent>(connection)
        .expect("Error loading events");

    assert_eq!(results.len(), 1);

    let saved_event = &results[0];

    assert_eq!(saved_event.urine, true);
    assert_eq!(saved_event.stool, true);
    assert_eq!(saved_event.skin2skin, 5);
    assert_eq!(saved_event.breastfeed, 10);
    assert_eq!(saved_event.breastmilk, 15);
    assert_eq!(saved_event.formula, 20);
    assert_eq!(saved_event.pump, 25);
}

/// Test reading events from the database.
///
/// This test writes multiple events to the database and then verifies their presence.
#[test]
fn test_read_events() {
    std::env::set_var("DATABASE_URL", ":memory:");

    let connection: &mut SqliteConnection = &mut establish_connection();

    run_migrations(connection).expect("Error running migrations");

    for i in 0..7 {
        let new_event: NewBabyEvent = create_event(
            Some(true),
            Some(true),
            Some(i),
            Some(i + 1),
            Some(i + 2),
            Some(i + 3),
            Some(i + 4),
        );
        write_event(connection, new_event);
    }

    let results: Vec<BabyEvent> = read_events(connection);

    assert_eq!(results.len(), 7);
}

/// Test updating an existing event in the database.
///
/// This test writes an event to the database, updates it, and then checks if the update is reflected.
#[test]
fn test_update_event() {
    use babyrs::schema::events::dsl::*;

    std::env::set_var("DATABASE_URL", ":memory:");

    let connection: &mut SqliteConnection = &mut establish_connection();

    run_migrations(connection).expect("Error running migrations");

    let new_event: NewBabyEvent = create_event(
        Some(true),
        Some(true),
        Some(5),
        Some(10),
        Some(15),
        Some(20),
        Some(25),
    );

    write_event(connection, new_event);

    let results: Vec<BabyEvent> = events
        .load::<BabyEvent>(connection)
        .expect("Error loading events");

    assert_eq!(results.len(), 1);

    let mut saved_event = results[0];

    saved_event.urine = false;
    saved_event.stool = false;
    saved_event.skin2skin = 0;
    saved_event.breastfeed = 0;
    saved_event.breastmilk = 0;
    saved_event.formula = 0;
    saved_event.pump = 0;

    assert_eq!(babyrs::update_event(connection, saved_event), 1);

    let results: Vec<BabyEvent> = events
        .load::<BabyEvent>(connection)
        .expect("Error loading events");

    assert_eq!(results.len(), 1);

    let updated_event = &results[0];

    assert_eq!(updated_event.urine, false);
    assert_eq!(updated_event.stool, false);
    assert_eq!(updated_event.skin2skin, 0);
    assert_eq!(updated_event.breastfeed, 0);
    assert_eq!(updated_event.breastmilk, 0);
    assert_eq!(updated_event.formula, 0);
    assert_eq!(updated_event.pump, 0);

    assert_eq!(updated_event.id, saved_event.id);
}

/// Test deleting an event from the database.
///
/// This test writes an event to the database, deletes it, and then verifies that it was deleted.
#[test]
fn test_delete_event() {
    use babyrs::schema::events::dsl::*;

    std::env::set_var("DATABASE_URL", ":memory:");

    let connection: &mut SqliteConnection = &mut establish_connection();

    run_migrations(connection).expect("Error running migrations");

    let new_event: NewBabyEvent = create_event(
        Some(true),
        Some(true),
        Some(5),
        Some(10),
        Some(15),
        Some(20),
        Some(25),
    );

    write_event(connection, new_event);

    let results: Vec<BabyEvent> = events
        .load::<BabyEvent>(connection)
        .expect("Error loading events");

    assert_eq!(results.len(), 1);

    let saved_event = results[0];

    assert_eq!(babyrs::delete_event(connection, saved_event), 1);

    let results: Vec<BabyEvent> = events
        .load::<BabyEvent>(connection)
        .expect("Error loading events");

    assert_eq!(results.len(), 0);
}

/// Test processing CSV files for event data.
///
/// This test reads events from a sample CSV file and writes them to the database, verifying their presence afterwards.
#[test]
fn test_process_csv() {
    use babyrs::schema::events::dsl::*;

    std::env::set_var("DATABASE_URL", ":memory:");

    let connection: &mut SqliteConnection = &mut establish_connection();

    run_migrations(connection).expect("Error running migrations");

    babyrs::process_csv(connection, "sample/example.csv").expect("Error processing CSV");

    let results: Vec<BabyEvent> = events
        .load::<BabyEvent>(connection)
        .expect("Error loading events");

    assert_eq!(results.len(), 12);

    let saved_event = &results[0];

    assert_eq!(saved_event.urine, false);
    assert_eq!(saved_event.stool, false);
    assert_eq!(saved_event.skin2skin, 60);
    assert_eq!(saved_event.breastfeed, 0);
    assert_eq!(saved_event.breastmilk, 0);
    assert_eq!(saved_event.formula, 0);
    assert_eq!(saved_event.pump, 0);
}
