use babyrs::{establish_connection, models::BabyEvent, read_events};
use diesel::sqlite::SqliteConnection;
use log::info;
use std::fmt::{self, Display};

/// Represents the filter for the event list.
///
/// The filter can be `Day`, `Week`, or `Month`.
#[derive(Debug, PartialEq)]
pub enum Filter {
    Day,
    Week,
    Month,
}

impl Filter {
    /// Returns the next filter in the sequence.
    ///
    /// # Parameters
    ///
    /// * `self`: The current filter.
    ///
    /// # Returns
    ///
    /// The next filter in the sequence.
    pub fn next(&self) -> Self {
        match self {
            Self::Day => Self::Week,
            Self::Week => Self::Month,
            Self::Month => Self::Day,
        }
    }
}

impl Default for Filter {
    fn default() -> Self {
        Self::Day
    }
}

impl Display for Filter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Day => write!(f, "Day"),
            Self::Week => write!(f, "Week"),
            Self::Month => write!(f, "Month"),
        }
    }
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

        Self::Initialized {
            baby_events,
            filter,
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
        if let Self::Initialized { baby_events, .. } = self {
            info!("Loading events from database...");

            // Establish connection to database
            let connection: &mut SqliteConnection = &mut establish_connection();
            *baby_events = read_events(connection);
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

    /// Updates the filter to the next value in the sequence.
    ///
    /// # Returns
    ///
    /// - `Some(Filter)` containing the filter if the state is `Initialized`.
    /// - `None` otherwise.
    pub fn update_filter(&mut self) {
        if let Self::Initialized { filter, .. } = self {
            *filter = filter.next();
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
        assert_eq!(state.get_filter().unwrap(), &Filter::Day);
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

        assert_eq!(state.update_filter(), ());
    }

    #[test]
    fn test_update_filter() {
        let mut state = AppState::initialized();

        assert_eq!(state.get_filter().unwrap(), &Filter::Day);

        state.update_filter();
        assert_eq!(state.get_filter().unwrap(), &Filter::Week);

        state.update_filter();
        assert_eq!(state.get_filter().unwrap(), &Filter::Month);

        state.update_filter();
        assert_eq!(state.get_filter().unwrap(), &Filter::Day);
    }

    #[test]
    fn test_next_filter() {
        assert_eq!(Filter::Day.next(), Filter::Week);
        assert_eq!(Filter::Week.next(), Filter::Month);
        assert_eq!(Filter::Month.next(), Filter::Day);
    }

    #[test]
    fn test_display_filter() {
        assert_eq!(format!("{}", Filter::Day), "Day");
        assert_eq!(format!("{}", Filter::Week), "Week");
        assert_eq!(format!("{}", Filter::Month), "Month");
    }
}
