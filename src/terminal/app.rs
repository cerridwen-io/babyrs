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
    pub state: AppState,
}

impl App {
    /// Constructs a new `App`.
    ///
    /// Initializes the app with default actions/state.
    ///
    /// # Returns
    ///
    /// A new `App` instance.
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        // This is the order of the actions displayed in the menu
        let actions = vec![
            Action::AddEvent,
            Action::DeleteEvent,
            Action::UpdateEvent,
            Action::NextEvent,
            Action::PreviousEvent,
            Action::SwitchFilter,
            Action::LoadCSV,
            Action::Quit,
        ]
        .into();
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
                Action::AddEvent => AppReturn::Continue,
                Action::DeleteEvent => AppReturn::Continue,
                Action::NextEvent => self.next_event(),
                Action::PreviousEvent => self.previous_event(),
                Action::SwitchFilter => self.switch_filter(),
                Action::LoadCSV => AppReturn::Continue,
                Action::UpdateEvent => AppReturn::Continue,
                Action::Quit => AppReturn::Exit,
            }
        } else {
            warn!("No action found for key: {:?}", key);
            AppReturn::Continue
        }
    }

    /// Initializes the application.
    ///
    /// # Returns
    ///
    /// An `AppReturn` indicating that the application should continue running.
    pub fn initialize(&mut self) -> AppReturn {
        self.state = AppState::initialized();
        AppReturn::Continue
    }

    /// Loads the events from the database into the state.
    ///
    /// # Returns
    ///
    /// An `AppReturn` indicating that the application should continue running.
    pub fn load_events(&mut self) -> AppReturn {
        self.state.load_events(None);
        AppReturn::Continue
    }

    /// Returns a reference to the application's actions.
    ///
    /// # Returns
    ///
    /// A reference to the application's actions.
    pub fn actions(&self) -> &Actions {
        &self.actions
    }

    /// Returns a reference to the application's state.
    ///
    /// # Returns
    ///
    /// A reference to the application's state.
    pub fn state(&mut self) -> &mut AppState {
        &mut self.state
    }

    /// Adds an event to the application.
    ///
    /// # Returns
    ///
    /// An `AppReturn` indicating that the application should continue running.
    pub fn add_event(&mut self) {
        // self.state.add_event();
        // AppReturn::Continue
        unimplemented!()
    }

    /// Deletes an event from the application.
    ///
    /// # Returns
    ///
    /// An `AppReturn` indicating that the application should continue running.
    pub fn delete_event(&mut self) {
        // self.state.delete_event();
        // AppReturn::Continue
        unimplemented!()
    }

    /// Move the event selection to the next event.
    ///
    /// # Returns
    ///
    /// An `AppReturn` indicating that the application should continue running.
    pub fn next_event(&mut self) -> AppReturn {
        self.state.increment_selection();
        AppReturn::Continue
    }

    /// Move the event selection to the previous event.
    ///
    /// # Returns
    ///
    /// An `AppReturn` indicating that the application should continue running.
    pub fn previous_event(&mut self) -> AppReturn {
        self.state.decrement_selection();
        AppReturn::Continue
    }

    /// Switches to the next filter.
    ///
    /// The order of filters is: day, week, month.
    /// If the current filter is month, it will wrap around to day.
    ///
    /// # Returns
    ///
    /// An `AppReturn` indicating that the application should continue running.
    pub fn switch_filter(&mut self) -> AppReturn {
        self.state.switch_filter();
        AppReturn::Continue
    }

    /// Loads events from a CSV file.
    pub fn load_csv(&mut self) {
        unimplemented!()
    }
}

/// Enum representing possible actions within the application.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Action {
    AddEvent,
    DeleteEvent,
    NextEvent,
    PreviousEvent,
    SwitchFilter,
    LoadCSV,
    UpdateEvent,
    Quit,
}

impl Action {
    /// Returns an iterator over all possible actions.
    ///
    /// # Returns
    ///
    /// An iterator over [`Action`].
    pub fn iterator() -> Iter<'static, Action> {
        static ACTIONS: [Action; 8] = [
            Action::AddEvent,
            Action::DeleteEvent,
            Action::UpdateEvent,
            Action::NextEvent,
            Action::PreviousEvent,
            Action::SwitchFilter,
            Action::LoadCSV,
            Action::Quit,
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
            Action::AddEvent => &[Key::Char('a')],
            Action::DeleteEvent => &[Key::Char('d')],
            Action::NextEvent => &[Key::Down],
            Action::PreviousEvent => &[Key::Up],
            Action::SwitchFilter => &[Key::Char('f')],
            Action::LoadCSV => &[Key::Char('l')],
            Action::UpdateEvent => &[Key::Char('u')],
            Action::Quit => &[Key::Char('q'), Key::Ctrl('c')],
        }
    }
}

impl Display for Action {
    /// Formats the `Action` for display purposes.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str = match self {
            Action::AddEvent => "add",
            Action::DeleteEvent => "delete",
            Action::NextEvent => "next",
            Action::PreviousEvent => "prev",
            Action::SwitchFilter => "switch filter",
            Action::LoadCSV => "load csv",
            Action::UpdateEvent => "update",
            Action::Quit => "quit",
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
