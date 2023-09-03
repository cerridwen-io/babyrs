use crate::terminal::events::Key;
use crate::terminal::state::AppState;
use log::{debug, warn};
use std::{
    collections::HashMap,
    fmt::{self, Display},
    slice::Iter,
};

#[derive(Debug, PartialEq, Eq)]
pub enum AppReturn {
    Exit,
    Continue,
}

pub struct App {
    actions: Actions,
    state: AppState,
}

impl App {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let actions = vec![Action::Quit].into();
        let state = AppState::default();

        Self { actions, state }
    }

    pub fn do_action(&mut self, key: Key) -> AppReturn {
        if let Some(action) = self.actions.find(key) {
            debug!("Action: {:?}", action);

            match action {
                Action::Quit => AppReturn::Exit,
                Action::AddEvent => AppReturn::Continue,
                Action::DeleteEvent => AppReturn::Continue,
                Action::UpdateEvent => AppReturn::Continue,
                Action::LoadCSV => AppReturn::Continue,
            }
        } else {
            warn!("No action found for key: {:?}", key);
            AppReturn::Continue
        }
    }

    pub fn update_tick(&mut self) -> AppReturn {
        self.state.increment_tick();
        AppReturn::Continue
    }

    pub fn actions(&self) -> &Actions {
        &self.actions
    }

    pub fn state(&self) -> &AppState {
        &self.state
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Action {
    Quit,
    AddEvent,
    DeleteEvent,
    UpdateEvent,
    LoadCSV,
}

impl Action {
    pub fn iterator() -> Iter<'static, Action> {
        static ACTIONS: [Action; 5] = [
            Action::Quit,
            Action::AddEvent,
            Action::DeleteEvent,
            Action::UpdateEvent,
            Action::LoadCSV,
        ];
        ACTIONS.iter()
    }

    pub fn keys(&self) -> &[Key] {
        match self {
            Action::Quit => &[Key::Ctrl('c'), Key::Char('q')],
            Action::AddEvent => &[Key::Char('a')],
            Action::DeleteEvent => &[Key::Char('d')],
            Action::UpdateEvent => &[Key::Char('u')],
            Action::LoadCSV => &[Key::Char('l')],
        }
    }
}

impl Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str = match self {
            Action::Quit => "Quit",
            Action::AddEvent => "Add Event",
            Action::DeleteEvent => "Delete Event",
            Action::UpdateEvent => "Update Event",
            Action::LoadCSV => "Load CSV",
        };
        write!(f, "{}", str)
    }
}

#[derive(Default, Debug, Clone)]
pub struct Actions(Vec<Action>);

impl Actions {
    pub fn find(&self, key: Key) -> Option<&Action> {
        Action::iterator()
            .filter(|action| self.0.contains(action))
            .find(|action| action.keys().contains(&key))
    }

    pub fn actions(&self) -> &[Action] {
        self.0.as_slice()
    }
}

impl From<Vec<Action>> for Actions {
    fn from(actions: Vec<Action>) -> Self {
        let mut map: HashMap<Key, Vec<Action>> = HashMap::new();

        for action in actions.iter() {
            for key in action.keys().iter() {
                match map.get_mut(key) {
                    Some(vec) => vec.push(*action),
                    None => {
                        map.insert(*key, vec![*action]);
                    }
                }
            }
        }

        let errors = map
            .iter()
            .filter(|(_, actions)| actions.len() > 1)
            .map(|(key, actions)| {
                let actions = actions
                    .iter()
                    .map(Action::to_string)
                    .collect::<Vec<_>>()
                    .join(", ");

                format!("Conflict key {} with actions {}", key, actions)
            })
            .collect::<Vec<_>>();

        if !errors.is_empty() {
            panic!("{}", errors.join("; "))
        }

        Self(actions)
    }
}
