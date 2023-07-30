use babyrs::models::Event;

pub enum AppState {
    Init,
    Initialized {
        counter_tick: u64,
        events: Vec<Event>,
    },
}

impl AppState {
    pub fn initialized() -> Self {
        let _events: Vec<Event> = Vec::new();
        let counter_tick = 0;

        Self::Initialized {
            counter_tick,
            events: Vec::new(),
        }
    }

    pub fn is_initialized(&self) -> bool {
        matches!(self, &Self::Initialized { .. })
    }

    pub fn increment_tick(&mut self) {
        if let Self::Initialized { counter_tick, .. } = self {
            *counter_tick += 1;
        }
    }

    pub fn count_tick(&self) -> Option<u64> {
        if let Self::Initialized { counter_tick, .. } = self {
            Some(*counter_tick)
        } else {
            None
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::Init
    }
}
