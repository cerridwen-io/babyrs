use crate::app::{Actions, App};
use crate::state::AppState;
use ratatui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Cell, Paragraph, Row, Table},
    Frame,
};

// use tui_logger::TuiLoggerWidget;

pub fn draw<B>(rect: &mut Frame<B>, app: &App)
where
    B: Backend,
{
    let size = rect.size();
    check_size(&size);

    // Vertical layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Min(10),
                Constraint::Length(3),
                Constraint::Length(12),
            ]
            .as_ref(),
        )
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

    // Logs
    // let logs = draw_logs();
    // rect.render_widget(logs, chunks[2]);
}

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

// fn draw_logs<'a>() -> TuiLoggerWidget<'a> {
//     TuiLoggerWidget::default()
//         .style_error(Style::default().fg(Color::Red))
//         .style_debug(Style::default().fg(Color::Green))
//         .style_warn(Style::default().fg(Color::Yellow))
//         .style_trace(Style::default().fg(Color::Gray))
//         .style_info(Style::default().fg(Color::Blue))
//         .block(
//             Block::default()
//                 .title("Logs")
//                 .border_style(Style::default().fg(Color::White).bg(Color::Black))
//                 .borders(Borders::ALL),
//         )
//         .style(Style::default().fg(Color::White).bg(Color::Black))
// }
