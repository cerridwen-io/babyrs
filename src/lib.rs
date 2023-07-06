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

pub fn create_event(
    urine: Option<bool>,
    stool: Option<bool>,
    skin2skin: Option<u16>,
    breastfeed: Option<u16>,
    breastmilk: Option<u16>,
    formula: Option<u16>,
    pump: Option<u16>,
) -> models::NewEvent {
    debug!("Creating event - urine: {:?}, stool: {:?}, skin2skin: {:?}, breastfeed: {:?}, breastmilk: {:?}, formula: {:?}, pump: {:?}",
        &urine, &stool, &skin2skin, &breastfeed, &breastmilk, &formula, &pump);

    models::NewEvent {
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

pub fn write_event(connection: &mut SqliteConnection, new_event: models::NewEvent) -> usize {
    debug!("Writing event: {:?}", &new_event);

    diesel::insert_into(schema::events::table)
        .values(&new_event)
        .execute(connection)
        .expect("Error saving new event")
}

pub fn read_events(connection: &mut SqliteConnection) -> Vec<models::Event> {
    use schema::events::dsl::*;

    debug!("Reading events");

    let results: Vec<models::Event> = events
        .select(models::Event::as_select())
        .load(connection)
        .expect("Error loading events");

    debug!("Read events: {:?}", &results);

    results
}

pub fn update_event(connection: &mut SqliteConnection, event: models::Event) -> usize {
    use schema::events::dsl::*;

    debug!("Updating event: {:?}", &event);

    diesel::update(events.find(event.id))
        .set(&event)
        .execute(connection)
        .expect("Error updating event")
}

pub fn delete_event(connection: &mut SqliteConnection, event: models::Event) -> usize {
    use schema::events::dsl::*;

    debug!("Deleting event: {:?}", &event);

    diesel::delete(events.find(event.id))
        .execute(connection)
        .expect("Error deleting event")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_database_url() {
        std::env::set_var(*DB_KEY, "sqlite://test.db");
        assert_eq!(get_database_url(), "sqlite://test.db");
    }

    #[test]
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
