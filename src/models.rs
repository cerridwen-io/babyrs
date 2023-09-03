/// This file contains the models for the database.
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::Deserialize;

/// Represents a baby event as it is stored in the database.
///
/// This struct is used for querying existing baby events from the database.
///
/// # Fields
///
/// - `id`: Unique identifier for the event.
/// - `dt`: The datetime when the event occurred.
/// - `urine`: Indicates if there was a urine event.
/// - `stool`: Indicates if there was a stool event.
/// - `skin2skin`: Duration in minutes of skin-to-skin contact.
/// - `breastfeed`: Duration in minutes of breastfeeding.
/// - `breastmilk`: Quantity of breastmilk consumed.
/// - `formula`: Quantity of formula consumed.
/// - `pump`: Duration in minutes of pumping.
#[derive(Queryable, Selectable, Debug, AsChangeset, Copy, Clone)]
#[diesel(table_name = crate::schema::events)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct BabyEvent {
    pub id: i32,
    pub dt: NaiveDateTime,
    pub urine: bool,
    pub stool: bool,
    pub skin2skin: i32,
    pub breastfeed: i32,
    pub breastmilk: i32,
    pub formula: i32,
    pub pump: i32,
}

/// Represents a new baby event to be inserted into the database.
///
/// This struct is used for creating new baby events.
///
/// # Fields
///
/// - `dt`: The datetime when the event occurred.
/// - `urine`: Indicates if there was a urine event.
/// - `stool`: Indicates if there was a stool event.
/// - `skin2skin`: Duration in minutes of skin-to-skin contact.
/// - `breastfeed`: Duration in minutes of breastfeeding.
/// - `breastmilk`: Quantity of breastmilk consumed.
/// - `formula`: Quantity of formula consumed.
/// - `pump`: Duration in minutes of pumping.
#[derive(Insertable, Debug, Deserialize)]
#[diesel(table_name = crate::schema::events)]
pub struct NewBabyEvent {
    pub dt: NaiveDateTime,
    pub urine: bool,
    pub stool: bool,
    pub skin2skin: i32,
    pub breastfeed: i32,
    pub breastmilk: i32,
    pub formula: i32,
    pub pump: i32,
}
