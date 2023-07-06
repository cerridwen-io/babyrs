use babyrs::{create_event, establish_connection, read_events, write_event};
use diesel::prelude::*;
use diesel::sqlite::Sqlite;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();
use babyrs::models::{Event, NewEvent};
use std::error::Error;

fn run_migrations(
    connection: &mut impl MigrationHarness<Sqlite>,
) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    connection.run_pending_migrations(MIGRATIONS)?;

    Ok(())
}

#[test]
fn test_establish_connection() {
    std::env::set_var("DATABASE_URL", ":memory:");

    let connection: &mut SqliteConnection = &mut establish_connection();

    assert!(diesel::sql_query("SELECT 1").execute(connection).is_ok());
}

#[test]
fn test_write_event() {
    use babyrs::schema::events::dsl::*;

    std::env::set_var("DATABASE_URL", ":memory:");

    let connection: &mut SqliteConnection = &mut establish_connection();

    run_migrations(connection).expect("Error running migrations");

    let new_event: NewEvent = create_event(
        Some(true),
        Some(true),
        Some(5),
        Some(10),
        Some(15),
        Some(20),
        Some(25),
    );

    assert_eq!(write_event(connection, new_event), 1);

    let results: Vec<Event> = events
        .load::<Event>(connection)
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

#[test]
fn test_read_events() {
    std::env::set_var("DATABASE_URL", ":memory:");

    let connection: &mut SqliteConnection = &mut establish_connection();

    run_migrations(connection).expect("Error running migrations");

    for i in 0..7 {
        let new_event: NewEvent = create_event(
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

    let results: Vec<Event> = read_events(connection);

    assert_eq!(results.len(), 7);
}

#[test]
fn test_update_event() {
    use babyrs::schema::events::dsl::*;

    std::env::set_var("DATABASE_URL", ":memory:");

    let connection: &mut SqliteConnection = &mut establish_connection();

    run_migrations(connection).expect("Error running migrations");

    let new_event: NewEvent = create_event(
        Some(true),
        Some(true),
        Some(5),
        Some(10),
        Some(15),
        Some(20),
        Some(25),
    );

    write_event(connection, new_event);

    let results: Vec<Event> = events
        .load::<Event>(connection)
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

    let results: Vec<Event> = events
        .load::<Event>(connection)
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

#[test]
fn test_delete_event() {
    use babyrs::schema::events::dsl::*;

    std::env::set_var("DATABASE_URL", ":memory:");

    let connection: &mut SqliteConnection = &mut establish_connection();

    run_migrations(connection).expect("Error running migrations");

    let new_event: NewEvent = create_event(
        Some(true),
        Some(true),
        Some(5),
        Some(10),
        Some(15),
        Some(20),
        Some(25),
    );

    write_event(connection, new_event);

    let results: Vec<Event> = events
        .load::<Event>(connection)
        .expect("Error loading events");

    assert_eq!(results.len(), 1);

    let saved_event = results[0];

    assert_eq!(babyrs::delete_event(connection, saved_event), 1);

    let results: Vec<Event> = events
        .load::<Event>(connection)
        .expect("Error loading events");

    assert_eq!(results.len(), 0);
}
