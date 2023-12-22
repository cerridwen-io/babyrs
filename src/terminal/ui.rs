use chrono::{Datelike, NaiveDateTime};
use ratatui::{
    prelude::*,
    widgets::{calendar::*, *},
    Frame,
};
use std::vec;
use time::{Date, Month};

use crate::terminal::app::{Actions, App};
use crate::terminal::state::{AppState, Filter};

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
pub fn draw_ui(rect: &mut Frame, app: &mut App) {
    let selection = app.state().get_selection().unwrap().selected();

    let size = rect.size();
    check_size(&size);

    // Vertical layout
    let vertical_chunks = Layout::new(
        Direction::Vertical,
        [Constraint::Length(3), Constraint::Min(0)],
    )
    .split(size);

    // Title and menu
    let title_and_menu = draw_title_and_menu(app.actions());
    rect.render_widget(title_and_menu, vertical_chunks[0]);

    // Horizontal layout for body
    let horizontal_chunks = Layout::new(
        Direction::Horizontal,
        Constraint::from_mins([24, size.width - 24]),
    )
    .split(vertical_chunks[1]);

    // Vertical layout for calendar and events
    let side_chunks = Layout::new(
        Direction::Vertical,
        Constraint::from_mins([9, vertical_chunks[1].height - 9]),
    )
    .split(horizontal_chunks[0]);

    // Calendar
    let calendar = draw_calendar(app.state());
    rect.render_widget(calendar, side_chunks[0]);

    // Event list
    let event_list = draw_event_list(app.state());
    rect.render_stateful_widget(
        event_list,
        side_chunks[1],
        app.state().get_selection().unwrap(),
    );

    // Vertical layout for details and graphing
    let data_chunks = Layout::new(
        Direction::Vertical,
        Constraint::from_mins([12, vertical_chunks[1].height - 12]),
    )
    .split(horizontal_chunks[1]);

    //horizontal layout for details
    let detail_chunks = Layout::new(
        Direction::Horizontal,
        Constraint::from_mins([25, size.width - 25]),
    )
    .split(data_chunks[0]);

    // Details
    let event_details = draw_event_details(app.state(), selection);
    rect.render_widget(event_details, detail_chunks[0]);

    // Statistics
    let statistics = draw_statistics(app.state());
    rect.render_widget(statistics, detail_chunks[1]);

    // Chart
    let chart = draw_chart(app.state());
    rect.render_widget(chart, data_chunks[1]);
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
    Table::new(
        vec![Row::new(menu_items)],
        &Constraint::from_mins([9, 12, 12, 13, 11, 19, 14, 10]),
    )
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Plain)
            .title(" Babyrs ")
            .title_style(Style::new().blue().bold()),
    )
    .column_spacing(1)
}

/// Creates a `Calendar` widget.
///
/// # Arguments
///
/// - `state`: Current `AppState` to display calendar.
///
/// # Returns
///
/// Returns a `Calendar` widget configured to display the calendar.
fn draw_calendar<'a>(
    state: &AppState,
) -> Monthly<'a, ratatui::widgets::calendar::CalendarEventStore> {
    let filter = state.get_filter().unwrap();
    let mut calendar_dates: CalendarEventStore = CalendarEventStore::default();

    // Get events based on the current filter enum
    let items = state
        .get_events()
        .unwrap()
        .iter()
        .filter(|e| match filter {
            Filter::Day(date) => &e.dt.date() == date,
            Filter::Week(week) => week
                .week(chrono::Weekday::Mon)
                .days()
                .contains(&e.dt.date()),
            Filter::Month(month) => e.dt.date().month() == month.month(),
        })
        .map(|e| e.dt)
        .collect::<Vec<NaiveDateTime>>();

    // add events to the calendar based on the filter and highlight them
    for date in &items {
        calendar_dates.add(convert_to_date(*date), Style::new().fg(Color::Yellow));
    }

    // get the current filter selection date
    let calendar_selection_date = match filter {
        Filter::Day(date) => *date,
        Filter::Week(week) => *week,
        Filter::Month(month) => *month,
    }
    .and_hms_opt(0, 0, 0)
    .unwrap();

    // add the current filter selection to the calendar and highlight it
    calendar_dates.add(
        convert_to_date(calendar_selection_date),
        Style::new().fg(Color::Green),
    );

    // construct the calendar widget
    Monthly::new(convert_to_date(calendar_selection_date), calendar_dates)
        .block(Block::new().padding(Padding::new(1, 1, 1, 1)))
        .show_month_header(Style::new().bold())
        .show_weekdays_header(Style::new().italic())
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
    // gather pre-filtered events
    let items = state
        .get_filtered_events()
        .unwrap()
        .iter()
        .map(|e| ListItem::new(format!("{}", e.dt)))
        .collect::<Vec<ListItem>>();

    // construct the list widget
    List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Plain)
                .title(" Events ")
                .title_style(Style::new().blue().bold()),
        )
        .highlight_style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ")
        .highlight_spacing(HighlightSpacing::Always)
        .direction(ListDirection::TopToBottom)
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
fn draw_event_details<'a>(state: &AppState, selection: Option<usize>) -> Paragraph<'a> {
    let event = selection.map(|i| state.get_filtered_events().unwrap()[i]);

    let text = match state {
        AppState::Init => "Welcome to babyrs! Press <q> to quit.".to_owned(),
        AppState::Initialized { .. } => match event {
            // TODO: is there a better way to construct a string that doesn't allocate to the heap? Also that isn't this ugly?
            Some(e) => format!("ID: {0} \n\rDate: {1} \n\rTime: {2} \n\rStool: {3} \n\rUrine: {4} \n\rSkin-to-Skin(min): {5} \n\rBreastfeed(min): {6} \n\rBreastmilk(ml): {7} \n\rFormula(ml): {8} \n\rPump(ml): {9}",
                e.id,
                e.dt.date(),
                e.dt.time(),
                e.stool,
                e.urine,
                e.skin2skin,
                e.breastfeed,
                e.breastmilk,
                e.formula,
                e.pump,
            )
            .to_owned(),
            None => "No event selected.".to_owned(),
        },
    };

    // construct the paragraph widget
    Paragraph::new(text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Plain)
                .title(" Event Details ")
                .title_style(Style::new().blue().bold()),
        )
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Left)
}

/// Creates a `Paragraph` widget containing statistics
///
/// # Arguments
///
/// - `state`: Current `AppState` to display statistics.
///
/// # Returns
///
/// Returns a `Paragraph` widget configured to display the statistics.
fn draw_statistics<'a>(state: &AppState) -> Paragraph<'a> {
    let text = match state {
        AppState::Init => "Not implemented...".to_owned(),
        AppState::Initialized { .. } => "Not Implemented...".to_owned(),
    };

    // construct the paragraph widget
    Paragraph::new(text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Plain)
                .title(format!(" {} Statistics ", state.get_filter().unwrap()))
                .title_style(Style::new().blue().bold()),
        )
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Left)
}

/// Creates a `BarChart` widget containing statistics
///
/// # Arguments
///
/// - `state`: Current `AppState` to display statistics.
///
/// # Returns
///
/// Returns a `BarChart` widget configured to display the statistics.
fn draw_chart<'a>(_state: &AppState) -> BarChart<'a> {
    // construct the BarChart widget
    BarChart::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Plain),
        )
        .style(Style::default().fg(Color::White))
        .bar_width(2)
        .data(&[("B0", 0), ("B1", 2), ("B2", 4), ("B3", 3)])
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
    if rect.width < 80 {
        panic!(
            "Terminal width too small, got {}; Please resize to at least 52 columns.",
            rect.width
        );
    }

    if rect.height < 24 {
        panic!(
            "Terminal height too small, got {}; Please resize to at least 28 rows.",
            rect.height
        );
    }
}

/// Convert NaiveDateTime to time::Date
///
/// # Arguments
///
/// - `dt`: NaiveDateTime to convert
///
/// # Returns
///
/// Returns a time::Date
fn convert_to_date(dt: NaiveDateTime) -> Date {
    Date::from_calendar_date(
        dt.year(),
        Month::try_from(dt.month() as u8).unwrap(),
        dt.day() as u8,
    )
    .unwrap()
}
