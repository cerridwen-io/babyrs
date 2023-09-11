use ratatui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
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
        .constraints([Constraint::Length(3), Constraint::Min(10)].as_ref())
        .split(size);

    // Title
    let title = draw_title();
    rect.render_widget(title, chunks[0]);

    // Body
    let body_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(20), Constraint::Length(32)].as_ref())
        .split(chunks[1]);

    let body = draw_body(false, app.state());
    rect.render_widget(body, body_chunks[0]);

    // Menu
    let menu = draw_menu(app.actions());
    rect.render_widget(menu, body_chunks[1]);

    // // Logs
    // let logs = draw_logs();
    // rect.render_widget(logs, chunks[2]);
}

/// Creates a `Paragraph` widget for the title.
///
/// # Returns
///
/// Returns a `Paragraph` widget configured to display the title.
fn draw_title<'a>() -> Paragraph<'a> {
    Paragraph::new("BabyRS")
        .style(Style::default().fg(Color::Yellow))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Plain),
        )
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

/// Creates a `Table` widget for the action menu.
///
/// # Arguments
///
/// - `actions`: The actions that can be performed in the application.
///
/// # Returns
///
/// Returns a `Table` widget configured to display the action menu.
fn draw_menu(actions: &Actions) -> Table {
    let key_style = Style::default().fg(Color::White);
    let menu_style = Style::default().fg(Color::White);

    let mut rows = vec![];

    for action in actions.actions().iter() {
        let mut first = true;

        for key in action.keys() {
            let menu = if first {
                first = false;
                action.to_string()
            } else {
                String::from("")
            };

            let row = Row::new(vec![
                Cell::from(Span::styled(key.to_string(), key_style)),
                Cell::from(Span::styled(menu, menu_style)),
            ]);

            rows.push(row);
        }
    }

    Table::new(rows)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Plain)
                .title("Menu"),
        )
        .widths(&[Constraint::Length(11), Constraint::Min(20)])
        .column_spacing(1)
}

// fn draw_logs<'a>() -> tui_logger::TuiLoggerWidget<'a> {
//     // tui_logger::TuiLoggerWidget::default()
//     //     // .style_error(Style::default().fg(Color::Red))
//     //     // .style_debug(Style::default().fg(Color::Green))
//     //     // .style_warn(Style::default().fg(Color::Yellow))
//     //     // .style_trace(Style::default().fg(Color::Magenta))
//     //     // .style_info(Style::default().fg(Color::Cyan))
//     //     .output_separator(':')
//     //     .output_timestamp(Some("%H:%M:%S".to_string()))
//     //     // .output_level(Some(TuiLoggerLevelOutput::Abbreviated))
//     //     .output_target(true)
//     //     .output_file(true)
//     //     .output_line(true)
// }

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
