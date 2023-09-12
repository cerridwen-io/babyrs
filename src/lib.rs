/// Module handling database operations for baby events.
///
/// This module provides functionalities for CRUD operations as well as processing CSV files.
use chrono::{Datelike, NaiveDate};
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

/// Calculate total volume of food consumed for each day.
///
/// # Arguments
///
/// - `events`: A vector of BabyEvent objects.
///
/// # Returns
///
/// A vector of tuples containing the date and total volume of food consumed.
pub fn calculate_daily_volume(events: Vec<BabyEvent>) -> Vec<(NaiveDate, i32)> {
    let mut daily_volume: Vec<(NaiveDate, i32)> = Vec::new();

    for event in events {
        let date = event.dt.date();
        let volume = event.breastmilk + event.formula;

        if let Some((_, tmp)) = daily_volume.iter_mut().find(|(d, _)| d == &date) {
            *tmp += volume;
        } else {
            daily_volume.push((date, volume));
        }
    }

    daily_volume
}

/// Calculate total volume of food consumed for each week, starting on Mondays.
///
/// # Arguments
///
/// - `daily_volume`: A vector of tuples containing the date and total volume of food consumed.
///
/// # Returns
///
/// A vector of tuples containing the start date of the week (Monday) and the total volume of food consumed for that week.
pub fn calculate_weekly_volume(daily_volume: Vec<(NaiveDate, i32)>) -> Vec<(NaiveDate, i32)> {
    let mut weekly_volume: Vec<(NaiveDate, i32)> = Vec::new();

    for (date, volume) in daily_volume {
        let days_from_monday = date.weekday().num_days_from_monday();
        let start_date = date - chrono::Duration::days(days_from_monday as i64);

        if let Some((_, tmp)) = weekly_volume.iter_mut().find(|(d, _)| d == &start_date) {
            *tmp += volume;
        } else {
            weekly_volume.push((start_date, volume));
        }
    }

    weekly_volume
}

/// Calculate total volume of food consumed for each month.
///
/// # Arguments
///
/// - `daily_volume`: A vector of tuples containing the date and total volume of food consumed.
///
/// # Returns
///
/// A vector of tuples containing the start date of the month and the total volume of food consumed for that month.
pub fn calculate_monthly_volume(daily_volume: Vec<(NaiveDate, i32)>) -> Vec<(NaiveDate, i32)> {
    let mut monthly_volume: Vec<(NaiveDate, i32)> = Vec::new();

    for (date, volume) in daily_volume {
        let start_date = NaiveDate::from_ymd_opt(date.year(), date.month(), 1).unwrap();

        if let Some((_, tmp)) = monthly_volume.iter_mut().find(|(d, _)| d == &start_date) {
            *tmp += volume;
        } else {
            monthly_volume.push((start_date, volume));
        }
    }

    monthly_volume
}

/// Calculate total volume of milk pumped for each day.
///
/// # Arguments
///
/// - `events`: A vector of BabyEvent objects.
///
/// # Returns
///
/// A vector of tuples containing the date and total volume of milk pumped.
pub fn calculate_daily_pumped(events: Vec<BabyEvent>) -> Vec<(NaiveDate, i32)> {
    let mut daily_pumped: Vec<(NaiveDate, i32)> = Vec::new();

    for event in events {
        let date = event.dt.date();
        let volume = event.pump;

        if let Some((_, tmp)) = daily_pumped.iter_mut().find(|(d, _)| d == &date) {
            *tmp += volume;
        } else {
            daily_pumped.push((date, volume));
        }
    }

    daily_pumped
}

/// Calculate total volume of milk pumped for each week, starting on Mondays.
///
/// # Arguments
///
/// - `daily_pumped`: A vector of tuples containing the date and total volume of milk pumped.
///
/// # Returns
///
/// A vector of tuples containing the start date of the week (Monday) and the total volume of milk pumped for that week.
pub fn calculate_weekly_pumped(daily_pumped: Vec<(NaiveDate, i32)>) -> Vec<(NaiveDate, i32)> {
    let mut weekly_pumped: Vec<(NaiveDate, i32)> = Vec::new();

    for (date, volume) in daily_pumped {
        let days_from_monday = date.weekday().num_days_from_monday();
        let start_date = date - chrono::Duration::days(days_from_monday as i64);

        if let Some((_, tmp)) = weekly_pumped.iter_mut().find(|(d, _)| d == &start_date) {
            *tmp += volume;
        } else {
            weekly_pumped.push((start_date, volume));
        }
    }

    weekly_pumped
}

/// Calculate total volume of milk pumped for each month.
///
/// # Arguments
///
/// - `daily_pumped`: A vector of tuples containing the date and total volume of milk pumped.
///
/// # Returns
///
/// A vector of tuples containing the start date of the month and the total volume of milk pumped for that month.
pub fn calculate_monthly_pumped(daily_pumped: Vec<(NaiveDate, i32)>) -> Vec<(NaiveDate, i32)> {
    let mut monthly_pumped: Vec<(NaiveDate, i32)> = Vec::new();

    for (date, volume) in daily_pumped {
        let start_date = NaiveDate::from_ymd_opt(date.year(), date.month(), 1).unwrap();

        if let Some((_, tmp)) = monthly_pumped.iter_mut().find(|(d, _)| d == &start_date) {
            *tmp += volume;
        } else {
            monthly_pumped.push((start_date, volume));
        }
    }

    monthly_pumped
}

/// Calculate number of wet diapers for each day.
///
/// # Arguments
///
/// - `events`: A vector of BabyEvent objects.
///
/// # Returns
///
/// A vector of tuples containing the date and number of wet diapers.
pub fn calculate_daily_wet_diapers(events: Vec<BabyEvent>) -> Vec<(NaiveDate, i32)> {
    let mut daily_wet_diapers: Vec<(NaiveDate, i32)> = Vec::new();

    for event in events {
        let date = event.dt.date();
        let wet_diapers = if event.urine { 1 } else { 0 };

        if let Some((_, tmp)) = daily_wet_diapers.iter_mut().find(|(d, _)| d == &date) {
            *tmp += wet_diapers;
        } else {
            daily_wet_diapers.push((date, wet_diapers));
        }
    }

    daily_wet_diapers
}

/// Calculate number of wet diapers for each week, starting on Mondays.
///
/// # Arguments
///
/// - `daily_wet_diapers`: A vector of tuples containing the date and number of wet diapers.
///
/// # Returns
///
/// A vector of tuples containing the start date of the week (Monday) and the number of wet diapers for that week.
pub fn calculate_weekly_wet_diapers(
    daily_wet_diapers: Vec<(NaiveDate, i32)>,
) -> Vec<(NaiveDate, i32)> {
    let mut weekly_wet_diapers: Vec<(NaiveDate, i32)> = Vec::new();

    for (date, wet_diapers) in daily_wet_diapers {
        let days_from_monday = date.weekday().num_days_from_monday();
        let start_date = date - chrono::Duration::days(days_from_monday as i64);

        if let Some((_, tmp)) = weekly_wet_diapers
            .iter_mut()
            .find(|(d, _)| d == &start_date)
        {
            *tmp += wet_diapers;
        } else {
            weekly_wet_diapers.push((start_date, wet_diapers));
        }
    }

    weekly_wet_diapers
}

/// Calculate number of poopy diapers for each day.
///
/// # Arguments
///
/// - `events`: A vector of BabyEvent objects.
///
/// # Returns
///
/// A vector of tuples containing the date and number of poopy diapers.
pub fn calculate_daily_poopy_diapers(events: Vec<BabyEvent>) -> Vec<(NaiveDate, i32)> {
    let mut daily_poopy_diapers: Vec<(NaiveDate, i32)> = Vec::new();

    for event in events {
        let date = event.dt.date();
        let poopy_diapers = if event.stool { 1 } else { 0 };

        if let Some((_, tmp)) = daily_poopy_diapers.iter_mut().find(|(d, _)| d == &date) {
            *tmp += poopy_diapers;
        } else {
            daily_poopy_diapers.push((date, poopy_diapers));
        }
    }

    daily_poopy_diapers
}

/// Calculate number of poopy diapers for each week, starting on Mondays.
///
/// # Arguments
///
/// - `daily_poopy_diapers`: A vector of tuples containing the date and number of poopy diapers.
///
/// # Returns
///
/// A vector of tuples containing the start date of the week (Monday) and the number of poopy diapers for that week.
pub fn calculate_weekly_poopy_diapers(
    daily_poopy_diapers: Vec<(NaiveDate, i32)>,
) -> Vec<(NaiveDate, i32)> {
    let mut weekly_poopy_diapers: Vec<(NaiveDate, i32)> = Vec::new();

    for (date, poopy_diapers) in daily_poopy_diapers {
        let days_from_monday = date.weekday().num_days_from_monday();
        let start_date = date - chrono::Duration::days(days_from_monday as i64);

        if let Some((_, tmp)) = weekly_poopy_diapers
            .iter_mut()
            .find(|(d, _)| d == &start_date)
        {
            *tmp += poopy_diapers;
        } else {
            weekly_poopy_diapers.push((start_date, poopy_diapers));
        }
    }

    weekly_poopy_diapers
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDateTime;

    use super::*;

    fn baby_events(
        date_time1: NaiveDateTime,
        date_time2: NaiveDateTime,
        date_time3: NaiveDateTime,
        date_time4: NaiveDateTime,
    ) -> Vec<BabyEvent> {
        vec![
            BabyEvent {
                id: 1,
                urine: true,
                stool: true,
                skin2skin: 0,
                breastfeed: 0,
                pump: 0,
                dt: date_time1,
                breastmilk: 100,
                formula: 50,
            },
            BabyEvent {
                id: 2,
                urine: true,
                stool: false,
                skin2skin: 0,
                breastfeed: 0,
                pump: 100,
                dt: date_time2,
                breastmilk: 0,
                formula: 150,
            },
            BabyEvent {
                id: 3,
                urine: false,
                stool: false,
                skin2skin: 0,
                breastfeed: 0,
                pump: 50,
                dt: date_time3,
                breastmilk: 50,
                formula: 50,
            },
            BabyEvent {
                id: 4,
                urine: false,
                stool: true,
                skin2skin: 0,
                breastfeed: 0,
                pump: 225,
                dt: date_time4,
                breastmilk: 50,
                formula: 50,
            },
        ]
    }

    /// Test to ensure get_database_url returns the correct database URL.
    #[test]
    fn test_get_database_url() {
        std::env::set_var(*DB_KEY, "sqlite://test.db");
        assert_eq!(get_database_url(), "sqlite://test.db");
    }

    /// Test to ensure create_event creates the event correctly.
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

    /// Test to ensure daily volume is calculated correctly.
    #[test]
    fn test_calculate_daily_volume() {
        let date_time1 = NaiveDate::from_ymd_opt(2023, 1, 1)
            .unwrap()
            .and_hms_opt(8, 0, 0)
            .unwrap();
        let date_time2 = NaiveDate::from_ymd_opt(2023, 1, 1)
            .unwrap()
            .and_hms_opt(10, 0, 0)
            .unwrap();
        let date_time3 = NaiveDate::from_ymd_opt(2023, 1, 2)
            .unwrap()
            .and_hms_opt(8, 0, 0)
            .unwrap();

        let events = baby_events(date_time1, date_time2, date_time3, date_time3);

        let result = calculate_daily_volume(events);

        assert_eq!(result.len(), 2);
        assert_eq!(result[0], (date_time1.date(), 300));
        assert_eq!(result[1], (date_time3.date(), 200));
    }

    /// Test to ensure weekly volume is calculated correctly.
    #[test]
    fn test_calculate_weekly_volume() {
        let monday = NaiveDate::from_ymd_opt(2023, 9, 4).unwrap();
        let tuesday = NaiveDate::from_ymd_opt(2023, 9, 5).unwrap();
        let wednesday = NaiveDate::from_ymd_opt(2023, 9, 6).unwrap();
        let next_monday = NaiveDate::from_ymd_opt(2023, 9, 11).unwrap();

        let daily_volumes = vec![
            (monday, 100),
            (tuesday, 200),
            (wednesday, 300),
            (next_monday, 400),
        ];

        let result = calculate_weekly_volume(daily_volumes);

        assert_eq!(result.len(), 2); // Expecting 2 consolidated weeks
        assert_eq!(result[0], (monday, 600)); // 600 is the total for the first week (monday + tuesday + wednesday)
        assert_eq!(result[1], (next_monday, 400)); // 400 for the next week
    }

    /// Test to ensure monthly volume is calculated correctly.
    #[test]
    fn test_calculate_monthly_volume() {
        // Setup
        let date1 = NaiveDate::from_ymd_opt(2023, 9, 1).unwrap();
        let date2 = NaiveDate::from_ymd_opt(2023, 9, 15).unwrap();
        let date3 = NaiveDate::from_ymd_opt(2023, 9, 30).unwrap();
        let date_next_month = NaiveDate::from_ymd_opt(2023, 10, 1).unwrap();

        let daily_volumes = vec![
            (date1, 100),
            (date2, 200),
            (date3, 300),
            (date_next_month, 400),
        ];

        // Action
        let result = calculate_monthly_volume(daily_volumes);

        // Assert
        assert_eq!(result.len(), 2); // Expecting 2 consolidated months
        assert_eq!(result[0], (date1, 600)); // 600 is the total for September
        assert_eq!(result[1], (date_next_month, 400)); // 400 for October
    }

    /// Test to ensure daily pumped volume is calculated correctly.
    #[test]
    fn test_calculate_daily_pumped() {
        let date_time1 = NaiveDate::from_ymd_opt(2023, 1, 1)
            .unwrap()
            .and_hms_opt(8, 0, 0)
            .unwrap();
        let date_time2 = NaiveDate::from_ymd_opt(2023, 1, 1)
            .unwrap()
            .and_hms_opt(10, 0, 0)
            .unwrap();
        let date_time3 = NaiveDate::from_ymd_opt(2023, 1, 2)
            .unwrap()
            .and_hms_opt(8, 0, 0)
            .unwrap();

        let events = baby_events(date_time1, date_time2, date_time3, date_time3);

        let result = calculate_daily_pumped(events);

        assert_eq!(result.len(), 2);
        assert_eq!(result[0], (date_time1.date(), 100));
        assert_eq!(result[1], (date_time3.date(), 275));
    }

    /// Test to ensure weekly pumped volume is calculated correctly.
    #[test]
    fn test_calculate_weekly_pumped() {
        let monday = NaiveDate::from_ymd_opt(2023, 9, 4).unwrap();
        let tuesday = NaiveDate::from_ymd_opt(2023, 9, 5).unwrap();
        let wednesday = NaiveDate::from_ymd_opt(2023, 9, 6).unwrap();
        let next_monday = NaiveDate::from_ymd_opt(2023, 9, 11).unwrap();

        let daily_volumes = vec![
            (monday, 100),
            (tuesday, 100),
            (wednesday, 50),
            (next_monday, 75),
        ];

        let result = calculate_weekly_pumped(daily_volumes);

        assert_eq!(result.len(), 2); // Expecting 2 consolidated weeks
        assert_eq!(result[0], (monday, 250)); // 250 is the total for the first week (monday + tuesday + wednesday)
        assert_eq!(result[1], (next_monday, 75)); // 75 for the next week
    }

    /// Test to ensure monthly pumped volume is calculated correctly.
    #[test]
    fn test_calculate_monthly_pumped() {
        // Setup
        let date1 = NaiveDate::from_ymd_opt(2023, 9, 1).unwrap();
        let date2 = NaiveDate::from_ymd_opt(2023, 9, 15).unwrap();
        let date3 = NaiveDate::from_ymd_opt(2023, 9, 30).unwrap();
        let date_next_month = NaiveDate::from_ymd_opt(2023, 10, 1).unwrap();

        let daily_volumes = vec![
            (date1, 50),
            (date2, 50),
            (date3, 30),
            (date_next_month, 100),
        ];

        // Action
        let result = calculate_monthly_volume(daily_volumes);

        // Assert
        assert_eq!(result.len(), 2); // Expecting 2 consolidated months
        assert_eq!(result[0], (date1, 130)); // 600 is the total for September
        assert_eq!(result[1], (date_next_month, 100)); // 400 for October
    }

    /// Test to ensure daily wet diapers are calculated correctly.
    #[test]
    fn test_calculate_daily_wet_diapers() {
        let date_time1 = NaiveDate::from_ymd_opt(2023, 1, 1)
            .unwrap()
            .and_hms_opt(8, 0, 0)
            .unwrap();
        let date_time2 = NaiveDate::from_ymd_opt(2023, 1, 1)
            .unwrap()
            .and_hms_opt(10, 0, 0)
            .unwrap();
        let date_time3 = NaiveDate::from_ymd_opt(2023, 1, 2)
            .unwrap()
            .and_hms_opt(8, 0, 0)
            .unwrap();

        let events = baby_events(date_time1, date_time2, date_time3, date_time3);

        let result = calculate_daily_wet_diapers(events);

        assert_eq!(result.len(), 2);
        assert_eq!(result[0], (date_time1.date(), 2));
        assert_eq!(result[1], (date_time3.date(), 0));
    }

    /// Test to ensure weekly wet diapers are calculated correctly.
    #[test]
    fn test_calculate_weekly_wet_diapers() {
        let monday = NaiveDate::from_ymd_opt(2023, 9, 4).unwrap();
        let tuesday = NaiveDate::from_ymd_opt(2023, 9, 5).unwrap();
        let wednesday = NaiveDate::from_ymd_opt(2023, 9, 6).unwrap();
        let next_monday = NaiveDate::from_ymd_opt(2023, 9, 11).unwrap();

        let daily_volumes = vec![(monday, 1), (tuesday, 2), (wednesday, 3), (next_monday, 4)];

        let result = calculate_weekly_wet_diapers(daily_volumes);

        assert_eq!(result.len(), 2); // Expecting 2 consolidated weeks
        assert_eq!(result[0], (monday, 6)); // 6 is the total for the first week (monday + tuesday + wednesday)
        assert_eq!(result[1], (next_monday, 4)); // 4 for the next week
    }

    /// Test to ensure daily poopy diapers are calculated correctly.
    #[test]
    fn test_calculate_daily_poopy_diapers() {
        let date_time1 = NaiveDate::from_ymd_opt(2023, 1, 1)
            .unwrap()
            .and_hms_opt(8, 0, 0)
            .unwrap();
        let date_time2 = NaiveDate::from_ymd_opt(2023, 1, 1)
            .unwrap()
            .and_hms_opt(10, 0, 0)
            .unwrap();
        let date_time3 = NaiveDate::from_ymd_opt(2023, 1, 2)
            .unwrap()
            .and_hms_opt(8, 0, 0)
            .unwrap();

        let events = baby_events(date_time1, date_time2, date_time3, date_time3);

        let result = calculate_daily_poopy_diapers(events);

        assert_eq!(result.len(), 2);
        assert_eq!(result[0], (date_time1.date(), 1));
        assert_eq!(result[1], (date_time3.date(), 1));
    }

    /// Test to ensure weekly poopy diapers are calculated correctly.
    #[test]
    fn test_calculate_weekly_poopy_diapers() {
        let monday = NaiveDate::from_ymd_opt(2023, 9, 4).unwrap();
        let tuesday = NaiveDate::from_ymd_opt(2023, 9, 5).unwrap();
        let wednesday = NaiveDate::from_ymd_opt(2023, 9, 6).unwrap();
        let next_monday = NaiveDate::from_ymd_opt(2023, 9, 11).unwrap();

        let daily_volumes = vec![(monday, 1), (tuesday, 2), (wednesday, 3), (next_monday, 4)];

        let result = calculate_weekly_poopy_diapers(daily_volumes);

        assert_eq!(result.len(), 2); // Expecting 2 consolidated weeks
        assert_eq!(result[0], (monday, 6)); // 6 is the total for the first week (monday + tuesday + wednesday)
        assert_eq!(result[1], (next_monday, 4)); // 4 for the next week
    }
}
