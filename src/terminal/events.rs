use crossterm::event;
use std::{
    fmt::{self, Display, Formatter},
    sync::mpsc::{channel, Receiver, RecvError, Sender},
    thread,
    time::Duration,
};

/// Represents an event in the system, which can either be a keypress or a tick.
pub enum InputEvent {
    Input(Key),
    Tick,
}

/// Manages the receiving of `InputEvent`s in a non-blocking manner.
pub struct Events {
    rx: Receiver<InputEvent>,
    _tx: Sender<InputEvent>,
}

impl Events {
    /// Creates a new `Events` instance.
    ///
    /// Spawns a new thread that listens for input and tick events, emitting them
    /// into a channel. The channel receiver is then returned wrapped in an `Events`
    /// object.
    ///
    /// # Parameters
    ///
    /// * `tick_rate`: The interval between emitting `Tick` events.
    ///
    /// # Returns
    ///
    /// A new `Events` instance that can be used to receive `InputEvent`s.
    pub fn new(tick_rate: Duration) -> Self {
        let (tx, rx) = channel();

        let event_tx = tx.clone();

        thread::spawn(move || loop {
            if crossterm::event::poll(tick_rate).unwrap() {
                if let event::Event::Key(key) = event::read().unwrap() {
                    let key = Key::from(key);
                    event_tx.send(InputEvent::Input(key)).unwrap();
                }
            }
            event_tx.send(InputEvent::Tick).unwrap();
        });

        Events { rx, _tx: tx }
    }

    /// Fetches the next `InputEvent` from the internal channel.
    ///
    /// # Returns
    ///
    /// * `Ok(InputEvent)` if an event is successfully received.
    /// * `Err(RecvError)` if the receiving channel is empty.
    pub fn next(&self) -> std::result::Result<InputEvent, RecvError> {
        self.rx.recv()
    }
}

/// Enum representing the various kinds of keys that can be pressed.
#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum Key {
    Enter,
    Tab,
    Backspace,
    Esc,
    Left,
    Right,
    Up,
    Down,
    /// A printable character.
    Char(char),
    /// A Ctrl+<key> combination.
    Ctrl(char),
    /// An Alt+<key> combination.
    Alt(char),
    /// An unknown key combination.
    Unknown,
}

impl Key {
    /// Checks if the key event should be considered an exit event.
    ///
    /// # Returns
    ///
    /// `true` if the key is an exit key (`Ctrl+C`, `q`, or `Esc`), otherwise `false`.
    pub fn is_exit(&self) -> bool {
        matches!(self, Key::Ctrl('c') | Key::Char('q') | Key::Esc)
    }
}

impl Display for Key {
    /// Formats the `Key` for display purposes.
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            Key::Alt(' ') => write!(f, "<Alt+Space>"),
            Key::Ctrl(' ') => write!(f, "<Ctrl+Space>"),
            Key::Char(' ') => write!(f, "<Space>"),
            Key::Alt(c) => write!(f, "<Alt+{}>", c),
            Key::Ctrl(c) => write!(f, "<Ctrl+{}>", c),
            Key::Char(c) => write!(f, "{}", c),
            _ => write!(f, "{:?}", self),
        }
    }
}

impl From<event::KeyEvent> for Key {
    /// Creates a `Key` instance from a crossterm `KeyEvent`.
    ///
    /// # Parameters
    ///
    /// * `key_event`: The crossterm `KeyEvent` to convert into a `Key`.
    ///
    /// # Returns
    ///
    /// A new `Key` instance corresponding to the given `KeyEvent`.
    fn from(key_event: event::KeyEvent) -> Self {
        match key_event {
            event::KeyEvent {
                code: event::KeyCode::Enter,
                ..
            } => Key::Enter,
            event::KeyEvent {
                code: event::KeyCode::Tab,
                ..
            } => Key::Tab,
            event::KeyEvent {
                code: event::KeyCode::Backspace,
                ..
            } => Key::Backspace,
            event::KeyEvent {
                code: event::KeyCode::Esc,
                ..
            } => Key::Esc,
            event::KeyEvent {
                code: event::KeyCode::Left,
                ..
            } => Key::Left,
            event::KeyEvent {
                code: event::KeyCode::Right,
                ..
            } => Key::Right,
            event::KeyEvent {
                code: event::KeyCode::Up,
                ..
            } => Key::Up,
            event::KeyEvent {
                code: event::KeyCode::Down,
                ..
            } => Key::Down,
            event::KeyEvent {
                code: event::KeyCode::Char(c),
                modifiers: event::KeyModifiers::ALT,
                ..
            } => Key::Alt(c),
            event::KeyEvent {
                code: event::KeyCode::Char(c),
                modifiers: event::KeyModifiers::CONTROL,
                ..
            } => Key::Ctrl(c),
            event::KeyEvent {
                code: event::KeyCode::Char(c),
                ..
            } => Key::Char(c),
            _ => Key::Unknown,
        }
    }
}
