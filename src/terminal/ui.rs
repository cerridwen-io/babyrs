use std::vec;

use ratatui::{
    prelude::*,
    widgets::{Block, BorderType, Borders, Cell, Paragraph, Row, Table},
    Frame,
};

use crate::terminal::app::{Actions, App};
use crate::terminal::state::AppState;

/// Renders the user interface.
///
/// This function draws the title, body, and menu on the terminal window.
///
/// # Arguments
///
/// - `rect`: The frame on which to draw the UI.
/// - `app`: The current application state.
///
/// # Type Parameters
///
/// - `B`: Represents the backend, must implement `Backend` trait.
pub fn draw_ui<B>(rect: &mut Frame<B>, app: &App)
where
    B: Backend,
{
    let size = rect.size();
    check_size(&size);

    // Vertical layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(size);

    // Title and menu
    let title_and_menu = draw_title_and_menu(app.actions());
    rect.render_widget(title_and_menu, chunks[0]);

    let body = draw_body(false, app.state());
    rect.render_widget(body, chunks[1]);
}

/// Creates a `Table` widget for the title and menu.
///
/// # Returns
///
/// Returns a `Table` widget configured to display the title and application menu.
fn draw_title_and_menu<'a>(actions: &Actions) -> Table<'a> {
    let mut menu_items = vec![];

    for action in actions.actions().iter() {
        menu_items.push(Cell::from(Line::from(vec![
            Span::styled(
                format!("<{}> ", action.keys()[0]),
                Style::default().fg(Color::Yellow),
            ),
            Span::styled(action.to_string(), Style::default().fg(Color::White)),
        ])));
    }

    // A single row with the menu items
    Table::new(vec![Row::new(menu_items)])
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Plain)
                .title("Babyrs")
                .title_style(Style::new().blue().bold()),
        )
        .widths(&[
            Constraint::Min(15),
            Constraint::Min(18),
            Constraint::Min(18),
            Constraint::Min(14),
            Constraint::Min(10),
        ])
        .column_spacing(1)
}

/// Creates a `Paragraph` widget for the body of the UI.
///
/// # Arguments
///
/// - `loading`: Indicates if the body should show a loading state.
/// - `state`: Current `AppState` to display the tick count.
///
/// # Returns
///
/// Returns a `Paragraph` widget configured to display the body content.
fn draw_body<'a>(loading: bool, state: &AppState) -> Paragraph<'a> {
    let loading_text = if loading { "Loading..." } else { "" };
    let tick_text = if let Some(ticks) = state.count_tick() {
        format!("Ticks: {}", ticks)
    } else {
        String::default()
    };

    Paragraph::new(vec![
        Line::from(Span::raw(loading_text)),
        Line::from(Span::raw(tick_text)),
    ])
    .style(Style::default().fg(Color::White))
    .alignment(Alignment::Left)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Plain),
    )
}

/// Validates the terminal size to ensure it meets minimum requirements.
///
/// # Arguments
///
/// - `rect`: The current terminal window size.
///
/// # Panics
///
/// This function will panic if the terminal size is too small.
fn check_size(rect: &Rect) {
    if rect.width < 52 {
        panic!(
            "Terminal width too small, got {}; Please resize to at least 52 columns.",
            rect.width
        );
    }

    if rect.height < 28 {
        panic!(
            "Terminal height too small, got {}; Please resize to at least 28 rows.",
            rect.height
        );
    }
}
