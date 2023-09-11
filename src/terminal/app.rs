use crate::terminal::events::Key;
use crate::terminal::state::AppState;
use log::{debug, warn};
use std::{
    collections::HashMap,
    fmt::{self, Display},
    slice::Iter,
};

/// Enum representing the state the application can be in after an action.
#[derive(Debug, PartialEq, Eq)]
pub enum AppReturn {
    /// The application should exit.
    Exit,
    /// The application should continue running.
    Continue,
}

/// The main application struct, holding all actions and state.
pub struct App {
    actions: Actions,
    state: AppState,
}

impl App {
    /// Constructs a new `App`.
    ///
    /// Initializes the app with default actions and state.
    ///
    /// # Returns
    ///
    /// A new `App` instance.
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let actions = vec![Action::Quit].into();
        let state = AppState::default();

        Self { actions, state }
    }

    /// Handles a key event by performing the associated action.
    ///
    /// # Parameters
    ///
    /// * `key`: The key for which an action should be performed.
    ///
    /// # Returns
    ///
    /// An `AppReturn` indicating whether to exit or continue the application.
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

    /// Updates the tick state.
    ///
    /// # Returns
    ///
    /// An `AppReturn` indicating that the application should continue running.
    pub fn update_tick(&mut self) -> AppReturn {
        self.state.increment_tick();
        AppReturn::Continue
    }

    /// Returns a reference to the application's actions.
    pub fn actions(&self) -> &Actions {
        &self.actions
    }

    /// Returns a reference to the application's state.
    pub fn state(&self) -> &AppState {
        &self.state
    }
}

/// Enum representing possible actions within the application.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Action {
    Quit,
    AddEvent,
    DeleteEvent,
    UpdateEvent,
    LoadCSV,
}

impl Action {
    /// Returns an iterator over all possible actions.
    ///
    /// # Returns
    ///
    /// An iterator over [`Action`].
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

    /// Returns the keys associated with this action.
    ///
    /// # Returns
    ///
    /// A slice of [`Key`] associated with the action.
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
    /// Formats the `Action` for display purposes.
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

/// A collection of [`Action`]s.
#[derive(Default, Debug, Clone)]
pub struct Actions(Vec<Action>);

impl Actions {
    /// Finds an action based on a key.
    ///
    /// # Parameters
    ///
    /// * `key`: The key to look for.
    ///
    /// # Returns
    ///
    /// An `Option` containing a reference to the action, if found.
    pub fn find(&self, key: Key) -> Option<&Action> {
        Action::iterator()
            .filter(|action| self.0.contains(action))
            .find(|action| action.keys().contains(&key))
    }

    /// Returns the actions as a slice.
    ///
    /// # Returns
    ///
    /// A slice of [`Action`].
    pub fn actions(&self) -> &[Action] {
        self.0.as_slice()
    }
}

impl From<Vec<Action>> for Actions {
    /// Creates an `Actions` instance from a vector of `Action`s.
    ///
    /// # Parameters
    ///
    /// * `actions`: A `Vec` containing actions to include in this collection.
    ///
    /// # Returns
    ///
    /// A new `Actions` instance containing the given actions.
    ///
    /// # Panics
    ///
    /// Panics if there are conflicting keys for different actions.
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