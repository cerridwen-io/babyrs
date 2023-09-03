use babyrs::models::BabyEvent;

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
        events: Vec<BabyEvent>,
    },
}

impl AppState {
    /// Creates a new `Initialized` state with default values.
    ///
    /// # Returns
    ///
    /// An `AppState::Initialized` variant with `counter_tick` set to 0 and an empty vector of events.
    pub fn initialized() -> Self {
        let _events: Vec<BabyEvent> = Vec::new();
        let counter_tick = 0;

        Self::Initialized {
            counter_tick,
            events: Vec::new(),
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
