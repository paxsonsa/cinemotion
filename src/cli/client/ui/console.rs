use tui::{
    backend::Backend,
    layout::{self, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph},
    Frame,
};

use unicode_width::UnicodeWidthStr;

use super::super::state::*;

pub fn render<B: Backend>(
    ctx: &UIMode,
    input: &String,
    messages: &Vec<String>,
    frame: &mut Frame<B>,
    area: Rect,
) {
    let style = match ctx {
        UIMode::Console => Style::default().fg(Color::Blue),
        _ => Style::default(),
    };
    let block = Block::default()
        .title(" Console ")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .style(style);
    frame.render_widget(block, area);

    let sections = Layout::default()
        .horizontal_margin(2)
        .vertical_margin(1)
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(1)].as_ref())
        .split(area);

    let items: Vec<_> = messages
        .iter()
        .rev()
        .map(|m| {
            let content = vec![Spans::from(Span::raw(format!("{}", m)))];
            ListItem::new(content)
        })
        .collect();
    let list = List::new(items)
        .block(Block::default())
        .start_corner(layout::Corner::BottomLeft);
    frame.render_widget(list, sections[0]);

    let cur_input = format!(">>> {}", input);
    let input = Paragraph::new(cur_input.clone()).style(
        Style::default()
            .fg(Color::White)
            .add_modifier(Modifier::BOLD),
    );
    frame.render_widget(input, sections[1]);

    if let UIMode::Console = ctx {
        frame.set_cursor(
            // Put cursor past the end of the input text
            sections[1].x + cur_input.width() as u16,
            // Move one line down, from the border to the input line
            sections[1].y,
        )
    }
}
