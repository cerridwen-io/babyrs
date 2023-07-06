use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Queryable, Selectable, Debug, AsChangeset, Copy, Clone)]
#[diesel(table_name = crate::schema::events)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Event {
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

#[derive(Insertable, Debug)]
#[diesel(table_name = crate::schema::events)]
pub struct NewEvent {
    pub dt: NaiveDateTime,
    pub urine: bool,
    pub stool: bool,
    pub skin2skin: i32,
    pub breastfeed: i32,
    pub breastmilk: i32,
    pub formula: i32,
    pub pump: i32,
}
