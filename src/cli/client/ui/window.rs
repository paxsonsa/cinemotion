use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

use super::super::state::*;

pub fn render<B: Backend>(frame: &mut Frame<B>, app: &mut UIState) {
    let sections = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(frame.size().height - 3), Constraint::Min(3)].as_ref())
        .split(frame.size());

    let main_frame = sections[0];
    let status_line = sections[1];

    let sections = Layout::default()
        .margin(1)
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(2), Constraint::Min(100)].as_ref())
        .split(main_frame);

    let header = sections[0];
    let mut text = Text::from("IndieMotion Client");
    text.patch_style(Style::default().add_modifier(Modifier::BOLD));
    let help_message = Paragraph::new(text);
    frame.render_widget(help_message, header);

    let body = sections[1];
    let sections = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(60), Constraint::Min(10)].as_ref())
        .split(body);

    let left_body = sections[0];
    let right_body = sections[1];

    let sections = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(left_body);

    let log = sections[0];
    let console = sections[1];

    super::console::render(&app.mode, &app.console, frame, console);
    super::log::render(&app.mode, &mut app.log, frame, log);

    let style = match app.mode {
        UIMode::Outliner => Style::default().fg(Color::Blue),
        _ => Style::default(),
    };
    let block = Block::default()
        .title(" Outline ")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .style(style);
    frame.render_widget(block, right_body);

    let items = vec![
        Span::raw("Press "),
        Span::styled("CTRL+d", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(" to exit, "),
        Span::styled("ESC", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(" to stop processing"),
    ];
    let text = Text::from(Spans::from(items));
    let help_message = Paragraph::new(text);
    frame.render_widget(help_message, status_line);
}
