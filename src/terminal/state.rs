use crate::{establish_connection, models::BabyEvent, read_events};
use chrono::{Datelike, NaiveDate};
use diesel::sqlite::SqliteConnection;
use log::info;
use ratatui::widgets::ListState;
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
    Week(NaiveDate),
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
            Self::Day(date) => Self::Week(*date),
            Self::Week(week) => Self::Month(*week),
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
        /// filtered event list.
        /// TODO: filtered_events should be a vector of references into baby_events
        filtered_events: Vec<BabyEvent>,
        /// The current selection offset for the filtered event list.
        selection: ListState,
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
        let filtered_events = vec![];
        let selection = ListState::default();

        Self::Initialized {
            baby_events,
            filter,
            filtered_events,
            selection,
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
    pub fn load_events(&mut self, connection: Option<&mut SqliteConnection>) {
        if let Self::Initialized {
            baby_events,
            filter,
            filtered_events,
            selection,
        } = self
        {
            info!("Loading events from database...");

            // Establish connection to database
            let mut local_connection;
            let conn = match connection {
                Some(c) => c,
                None => {
                    local_connection = establish_connection();
                    &mut local_connection
                }
            };

            // let connection: &mut SqliteConnection = &mut establish_connection();
            *baby_events = read_events(conn);

            // initialize the filter to the latest event (day)
            *filter = Filter::Day(baby_events.last().unwrap().dt.date());

            // initialize the filtered events to the last day
            // TODO: simplify the match to just be the day filter (we know it won't ever be week or month at this point)
            *filtered_events = baby_events
                .clone()
                .into_iter()
                .filter(|e| match filter {
                    Filter::Day(date) => &e.dt.date() == date,
                    Filter::Week(week) => week
                        .week(chrono::Weekday::Mon)
                        .days()
                        .contains(&e.dt.date()),
                    Filter::Month(month) => e.dt.date().month() == month.month(),
                })
                .collect::<Vec<BabyEvent>>();

            // reset the selection offset
            *selection = ListState::default();
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

    /// Switches the filter to the next filter in the sequence. Switching the filter resets the selection offset to 0.
    /// It also recalculates the filtered events based on the new filter.
    ///
    /// # Returns
    ///
    /// - `Some(Filter)` containing the filter if the state is `Initialized`.
    /// - `None` otherwise.
    pub fn switch_filter(&mut self) {
        if let Self::Initialized {
            baby_events,
            filter,
            filtered_events,
            selection,
            ..
        } = self
        {
            // switch the filter
            *filter = filter.switch();

            // recalculate the filtered events
            *filtered_events = baby_events
                .clone()
                .into_iter()
                .filter(|e| match filter {
                    Filter::Day(date) => &e.dt.date() == date,
                    Filter::Week(week) => week
                        .week(chrono::Weekday::Mon)
                        .days()
                        .contains(&e.dt.date()),
                    Filter::Month(month) => e.dt.date().month() == month.month(),
                })
                .collect::<Vec<BabyEvent>>();

            // reset the selection offset
            *selection = ListState::default();
        }
    }

    /// Returns the current value of `filtered_events` if the state is `Initialized`.
    /// TODO: filtered_events should be a vector of references into baby_events
    ///
    /// # Returns
    ///
    /// - `Some(Vec<BabyEvent>)` containing the filtered events if the state is `Initialized`.
    /// - `None` otherwise.
    pub fn get_filtered_events(&self) -> Option<&Vec<BabyEvent>> {
        if let Self::Initialized {
            filtered_events, ..
        } = self
        {
            Some(filtered_events)
        } else {
            None
        }
    }

    /// Returns the current value of `selection` if the state is `Initialized`.
    ///
    /// # Returns
    ///
    /// - `Some(usize)` containing the selection offset if the state is `Initialized`.
    /// - `None` otherwise.
    pub fn get_selection(&mut self) -> Option<&mut ListState> {
        if let Self::Initialized { selection, .. } = self {
            Some(selection)
        } else {
            None
        }
    }

    /// Increments the selection offset by 1 if the state is `Initialized`. Loops back to the beginning of the list if
    /// the selection offset is already at the end of the list.
    ///
    /// # Returns
    ///
    /// - `Some(usize)` containing the selection offset if the state is `Initialized`.
    /// - `None` otherwise.
    pub fn increment_selection(&mut self) {
        if let Self::Initialized {
            selection,
            filtered_events,
            ..
        } = self
        {
            if filtered_events.is_empty() {
                selection.select(None);
            } else {
                let i = match selection.selected() {
                    Some(i) => {
                        if i >= filtered_events.len() - 1 {
                            0
                        } else {
                            i + 1
                        }
                    }
                    None => 0,
                };

                selection.select(Some(i));
            }
        }
    }

    /// Decrements the selection offset by 1 if the state is `Initialized`. Loops back to the end of the list if the
    /// selection offset is already at the beginning of the list.
    ///
    /// # Returns
    ///
    /// - `Some(usize)` containing the selection offset if the state is `Initialized`.
    /// - `None` otherwise.
    pub fn decrement_selection(&mut self) {
        if let Self::Initialized {
            selection,
            filtered_events,
            ..
        } = self
        {
            if filtered_events.is_empty() {
                selection.select(None);
            } else {
                let i = match selection.selected() {
                    Some(i) => {
                        if i == 0 {
                            filtered_events.len() - 1
                        } else {
                            i - 1
                        }
                    }
                    None => 0,
                };

                selection.select(Some(i));
            }
        }
    }

    /// Unselects the current selected item if any. Implementation of `ListState` ensures that the stored offset is
    /// also reset.
    ///
    /// # Returns
    ///
    /// - `Some(usize)` containing the selection offset if the state is `Initialized`.
    /// - `None` otherwise.
    pub fn unselect(&mut self) {
        if let Self::Initialized { selection, .. } = self {
            selection.select(None);
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
        assert!(state.get_filtered_events().unwrap().is_empty());
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

        state.load_events(None);
    }

    #[test]
    fn test_get_filter_not_initialized() {
        let state = AppState::default();

        assert!(state.get_filter().is_none());
    }

    #[test]
    fn test_switch_filter_not_initialized() {
        let mut state = AppState::default();

        assert_eq!(state.switch_filter(), ());
    }

    #[test]
    fn test_switch_filter() {
        let mut state = AppState::initialized();
        let test_date = NaiveDate::default();

        // test that the initialized state has the default filter
        assert_eq!(state.get_filter().unwrap(), &Filter::Day(test_date));

        // test switching from day to week
        state.switch_filter();
        assert_eq!(state.get_filter().unwrap(), &Filter::Week(test_date));

        // test switching from week to month
        state.switch_filter();
        assert_eq!(state.get_filter().unwrap(), &Filter::Month(test_date));

        // test switching from month to day
        state.switch_filter();
        assert_eq!(state.get_filter().unwrap(), &Filter::Day(test_date));
    }

    #[test]
    fn test_display_filter() {
        let test_date = NaiveDate::default();

        assert_eq!(format!("{}", Filter::Day(test_date)), "Day");
        assert_eq!(format!("{}", Filter::Week(test_date)), "Week");
        assert_eq!(format!("{}", Filter::Month(test_date)), "Month");
    }

    #[test]
    fn test_unselect() {
        let mut state = AppState::initialized();

        state.unselect();

        assert!(state.get_selection().unwrap().selected().is_none());
    }

    #[test]
    fn test_unselect_not_initialized() {
        let mut state = AppState::default();

        state.unselect();

        assert!(state.get_selection().is_none());
    }
}
