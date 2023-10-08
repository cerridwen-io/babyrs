use std::vec;

use ratatui::{
    prelude::*,
    widgets::{Block, BorderType, Borders, Cell, List, ListItem, Paragraph, Row, Table},
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
    let vertical_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(size);

    // Title and menu
    let title_and_menu = draw_title_and_menu(app.actions());
    rect.render_widget(title_and_menu, vertical_chunks[0]);

    // Horizontal layout for body
    let horizontal_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
        .split(vertical_chunks[1]);

    // Event list
    let event_list = draw_event_list(app.state());
    rect.render_widget(event_list, horizontal_chunks[0]);

    // Details
    let details = draw_details(app.state());
    rect.render_widget(details, horizontal_chunks[1]);
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

/// Creates a `List` widget containing baby_event datetime values.
///
/// # Arguments
///
/// - `state`: Current `AppState` to display baby_events.
///
/// # Returns
///
/// Returns a `List` widget configured to display the body content.
fn draw_event_list<'a>(state: &AppState) -> List<'a> {
    let mut items = vec![];

    for baby_event in state.get_events().unwrap().iter() {
        items.push(ListItem::new(format!("{}", baby_event.dt)));
    }

    assert_eq!(12, items.len());

    List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Plain)
                .title("Events")
                .title_style(Style::new().blue().bold()),
        )
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol(">> ")
        .start_corner(Corner::TopLeft)
        .style(Style::default().fg(Color::White))
}

/// Creates a `Paragraph` widget containing AppState baby_event details.
///
/// # Arguments
///
/// - `state`: Current `AppState` to display details.
///
/// # Returns
///
/// Returns a `Paragraph` widget configured to display the details.
fn draw_details<'a>(state: &AppState) -> Paragraph<'a> {
    let text = match state {
        AppState::Init => "Welcome to babyrs! Press <q> to quit.",
        AppState::Initialized { .. } => "DETAILS",
    };

    Paragraph::new(text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Plain)
                .title("Details")
                .title_style(Style::new().blue().bold()),
        )
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Left)
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
