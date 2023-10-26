use babyrs::{establish_connection, models::BabyEvent, read_events};
use chrono::{Datelike, IsoWeek, NaiveDate};
use diesel::sqlite::SqliteConnection;
use log::info;
use std::{
    fmt::{self, Display},
    vec,
};

/// Represents the filter for the event list.
///
/// The filter can be `Day`, `Week`, or `Month`.
#[derive(Debug, PartialEq)]
pub enum Filter {
    Day(NaiveDate),
    Week(IsoWeek),
    Month(NaiveDate),
}

impl Filter {
    /// Switches to the next filter in the sequence.
    ///
    /// # Parameters
    ///
    /// * `self`: The current filter.
    ///
    /// # Returns
    ///
    /// The next filter in the sequence.
    pub fn switch(&self) -> Self {
        match self {
            Self::Day(date) => Self::Week(date.iso_week()),
            Self::Week(week) => Self::Month(iso_week_to_naive_date(*week).unwrap()),
            Self::Month(month) => Self::Day(*month),
        }
    }
}

impl Default for Filter {
    fn default() -> Self {
        Self::Day(NaiveDate::default())
    }
}

impl Display for Filter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Day(_) => write!(f, "Day"),
            Self::Week(_) => write!(f, "Week"),
            Self::Month(_) => write!(f, "Month"),
        }
    }
}

/// Convert an `IsoWeek` to a month. in `NaiveDate` format. The month is set to the first day of the month.
///
/// # Parameters
///
/// * `week`: The `IsoWeek` to convert.
///
/// # Returns
///
/// The `NaiveDate` corresponding to the first day of the month.
fn iso_week_to_naive_date(week: IsoWeek) -> Option<NaiveDate> {
    let first_day_of_iso_year = NaiveDate::from_isoywd_opt(week.year(), 1, chrono::Weekday::Mon)?;
    let january_1st_weekday = first_day_of_iso_year.weekday();

    let days_to_add = match january_1st_weekday {
        chrono::Weekday::Mon => (week.week() - 1) as i64 * 7,
        _ => {
            // Handle the case where January 1st is part of the first ISO week of the year.
            let days_to_subtract = (7 - january_1st_weekday.num_days_from_monday()) as i64;
            (week.week() - 2) as i64 * 7 - days_to_subtract
        }
    };

    Some(first_day_of_iso_year + chrono::Duration::days(days_to_add))
}

/// Represents the application state.
///
/// The state can either be `Init` for the initial state,
/// or `Initialized` when the application is running and has data.
pub enum AppState {
    /// Initial state of the application.
    Init,
    /// State of the application when it is running and has data.
    Initialized {
        /// The events that have been added to the application.
        baby_events: Vec<BabyEvent>,
        /// The filter for the event list.
        filter: Filter,
        event_selection: Vec<BabyEvent>,
    },
}

impl AppState {
    /// Creates a new `Initialized` state with default values.
    ///
    /// # Returns
    ///
    /// An `AppState::Initialized` variant with an empty vector of events and the default filter.
    pub fn initialized() -> Self {
        let baby_events = vec![];
        let filter = Filter::default();
        let event_selection: Vec<BabyEvent> = vec![];

        Self::Initialized {
            baby_events,
            filter,
            event_selection,
        }
    }

    /// Checks if the current state is `Initialized`.
    ///
    /// # Returns
    ///
    /// - `true` if the state is `Initialized`.
    /// - `false` otherwise.
    pub fn is_initialized(&self) -> bool {
        matches!(self, &Self::Initialized { .. })
    }

    /// Loads the events from the database into the state.
    ///
    /// Does nothing if the state is not `Initialized`.
    pub fn load_events(&mut self) {
        if let Self::Initialized {
            baby_events,
            event_selection,
            filter,
            ..
        } = self
        {
            info!("Loading events from database...");

            // Establish connection to database
            let connection: &mut SqliteConnection = &mut establish_connection();
            *baby_events = read_events(connection);

            // update the day/week/month selection to the latest event
            *event_selection = vec![*baby_events.last().unwrap()];

            // set the filter to the latest event
            *filter = Filter::Day(baby_events.last().unwrap().dt.date());
        }
    }

    /// Returns the current value of `baby_events` if the state is `Initialized`.
    ///
    /// # Returns
    ///
    /// - `Some(Vec<BabyEvent>)` containing the events if the state is `Initialized`.
    /// - `None` otherwise.
    pub fn get_events(&self) -> Option<&Vec<BabyEvent>> {
        if let Self::Initialized { baby_events, .. } = self {
            Some(baby_events)
        } else {
            None
        }
    }

    /// Returns the current value of `filter` if the state is `Initialized`.
    ///
    /// # Returns
    ///
    /// - `Some(Filter)` containing the filter if the state is `Initialized`.
    /// - `None` otherwise.
    pub fn get_filter(&self) -> Option<&Filter> {
        if let Self::Initialized { filter, .. } = self {
            Some(filter)
        } else {
            None
        }
    }

    /// Switches the filter to the next filter in the sequence.
    ///
    /// # Returns
    ///
    /// - `Some(Filter)` containing the filter if the state is `Initialized`.
    /// - `None` otherwise.
    pub fn switch_filter(&mut self) {
        if let Self::Initialized { filter, .. } = self {
            *filter = filter.switch();
        }
    }
}

/// Implements the `Default` trait for `AppState`.
///
/// The default state is `AppState::Init`.
impl Default for AppState {
    fn default() -> Self {
        Self::Init
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialized() {
        let state = AppState::initialized();

        assert!(state.is_initialized());
        assert!(state.get_events().unwrap().is_empty());
        assert_eq!(
            state.get_filter().unwrap(),
            &Filter::Day(NaiveDate::default())
        );
    }

    #[test]
    fn test_get_events_not_initialized() {
        let state = AppState::default();

        assert!(state.get_events().is_none());
    }

    #[test]
    fn test_load_events_not_initialized() {
        let mut state = AppState::default();

        state.load_events();
    }

    #[test]
    fn test_get_filter_not_initialized() {
        let state = AppState::default();

        assert!(state.get_filter().is_none());
    }

    #[test]
    fn test_update_filter_not_initialized() {
        let mut state = AppState::default();

        assert_eq!(state.switch_filter(), ());
    }

    #[test]
    fn test_switch_filter() {
        let mut state = AppState::initialized();

        // test that the initialized state has the default filter
        assert_eq!(
            state.get_filter().unwrap(),
            &Filter::Day(NaiveDate::default())
        );

        // test switching from day to week
        state.switch_filter();
        assert_eq!(
            state.get_filter().unwrap(),
            &Filter::Week(NaiveDate::default().iso_week())
        );

        // switching NaiveDate::default() from week to month results in the last month of the previous year in IsoWeek format
        let start_of_the_week = NaiveDate::from_ymd_opt(1969, 12, 29).unwrap();

        // test switching from week to month
        state.switch_filter();
        assert_eq!(
            state.get_filter().unwrap(),
            &Filter::Month(start_of_the_week)
        );

        // test switching from month to day
        state.switch_filter();
        assert_eq!(state.get_filter().unwrap(), &Filter::Day(start_of_the_week));
    }

    #[test]
    fn test_display_filter() {
        assert_eq!(format!("{}", Filter::Day(NaiveDate::default())), "Day");
        assert_eq!(
            format!("{}", Filter::Week(NaiveDate::default().iso_week())),
            "Week"
        );
        assert_eq!(format!("{}", Filter::Month(NaiveDate::default())), "Month");
    }

    #[test]
    fn test_iso_week_to_naive_date() {
        // Test the the last iso week of the year converts to the appropriate Monday
        assert_eq!(
            iso_week_to_naive_date(NaiveDate::from_ymd_opt(2023, 1, 1).unwrap().iso_week()),
            NaiveDate::from_ymd_opt(2022, 12, 26)
        );

        // Test that the first iso week of the year converts to the appropriate Monday
        for day in 2..=8 {
            assert_eq!(
                iso_week_to_naive_date(NaiveDate::from_ymd_opt(2023, 1, day).unwrap().iso_week()),
                NaiveDate::from_ymd_opt(2023, 1, 2)
            );
        }

        // Test that the second iso week of the year converts to the appropriate Monday
        assert_eq!(
            iso_week_to_naive_date(NaiveDate::from_ymd_opt(2023, 1, 9).unwrap().iso_week()),
            NaiveDate::from_ymd_opt(2023, 1, 9)
        );
    }
}
