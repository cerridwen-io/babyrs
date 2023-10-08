use babyrs::{establish_connection, models::BabyEvent, read_events};
use diesel::sqlite::SqliteConnection;
use log::info;

/// Represents the application state.
///
/// The state can either be `Init` for the initial state,
/// or `Initialized` when the application is running and has data.
pub enum AppState {
    /// Initial state of the application.
    Init,
    /// State of the application when it is running and has data.
    Initialized {
        counter_tick: u64,
        /// The events that have been added to the application.
        baby_events: Vec<BabyEvent>,
    },
}

impl AppState {
    /// Creates a new `Initialized` state with default values.
    ///
    /// # Returns
    ///
    /// An `AppState::Initialized` variant with `counter_tick` set to 0 and an empty vector of events.
    pub fn initialized() -> Self {
        let counter_tick = 0;
        let baby_events = vec![];

        Self::Initialized {
            counter_tick,
            baby_events,
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

    /// Increments the `counter_tick` field by 1 if the state is `Initialized`.
    ///
    /// Does nothing if the state is not `Initialized`.
    pub fn increment_tick(&mut self) {
        if let Self::Initialized { counter_tick, .. } = self {
            *counter_tick += 1;
        }
    }

    /// Returns the current value of `counter_tick` if the state is `Initialized`.
    ///
    /// # Returns
    ///
    /// - `Some(u64)` containing the tick count if the state is `Initialized`.
    /// - `None` otherwise.
    pub fn count_tick(&self) -> Option<u64> {
        if let Self::Initialized { counter_tick, .. } = self {
            Some(*counter_tick)
        } else {
            None
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
        assert_eq!(state.count_tick(), Some(0));
        assert!(state.get_events().unwrap().is_empty());
    }

    #[test]
    fn test_increment_tick() {
        let mut state = AppState::initialized();

        assert_eq!(state.count_tick(), Some(0));

        state.increment_tick();
        assert_eq!(state.count_tick(), Some(1));

        state.increment_tick();
        assert_eq!(state.count_tick(), Some(2));
    }

    #[test]
    fn test_count_tick_not_initialized() {
        let state = AppState::default();

        assert_eq!(state.count_tick(), None);
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
}
