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
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};
use std::time::Duration;
use ui::draw_ui;

pub fn start_ui(mut app: App) -> Result<(), Box<dyn std::error::Error>> {
    // Setup terminal
    enable_raw_mode()?;
    // let mut stdout: std::io::Stdout = stdout();
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
    // terminal.clear()?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> Result<(), Box<dyn std::error::Error>> {
    let tick_rate = Duration::from_millis(200);
    let events = Events::new(tick_rate);

    loop {
        terminal.draw(|f| draw_ui(f, app))?;

        let result = match events.next()? {
            InputEvent::Input(key) => app.do_action(key),
            InputEvent::Tick => app.update_tick(),
        };

        if result == AppReturn::Exit {
            return Ok(());
        }
    }
}
