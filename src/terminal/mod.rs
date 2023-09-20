pub mod app;
mod events;
mod state;
mod ui;

use app::{App, AppReturn};
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use events::{Events, InputEvent};
use ratatui::prelude::*;
use std::time::Duration;
use ui::draw_ui;

/// Starts the user interface for the application.
///
/// # Parameters
///
/// * `app`: The `App` instance that holds the state and actions.
///
/// # Returns
///
/// Returns a `Result` which is `Ok` if the UI starts successfully,
/// or an `Err` containing an error.
pub fn start_ui(mut app: App) -> Result<(), Box<dyn std::error::Error>> {
    // Setup terminal
    enable_raw_mode()?;
    execute!(std::io::stderr(), EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(std::io::stderr());
    let mut terminal = Terminal::new(backend)?;

    terminal.clear()?;
    terminal.hide_cursor()?;

    // Create app and run
    let res = run_app(&mut terminal, &mut app);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;

    terminal.show_cursor()?;

    // Log any errors encountered while running the app
    if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

/// Runs the app within a terminal.
///
/// # Type Parameters
///
/// * `B`: Backend for the terminal, implementing `ratatui::backend::Backend`.
///
/// # Parameters
///
/// * `terminal`: Mutable reference to the terminal.
/// * `app`: Mutable reference to the `App` instance.
///
/// # Returns
///
/// Returns a `Result` which is `Ok` if the app runs successfully,
/// or an `Err` containing an error.
fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> Result<(), Box<dyn std::error::Error>> {
    let tick_rate = Duration::from_millis(200);
    let events = Events::new(tick_rate);

    // Main event loop
    loop {
        // Draw the user interface
        terminal.draw(|f| draw_ui(f, app))?;

        // Handle input events
        let result = match events.next()? {
            InputEvent::Input(key) => app.do_action(key),
            InputEvent::Tick => app.update_tick(),
        };

        // Exit if the app returns an exit code
        if result == AppReturn::Exit {
            return Ok(());
        }
    }
}
