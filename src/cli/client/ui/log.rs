use tui::{
    backend::Backend,
    layout::{self, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph},
    Frame,
};

use super::super::state::*;

pub fn render<B: Backend>(ctx: &UIMode, log: &mut LogState, frame: &mut Frame<B>, area: Rect) {
    let style = match ctx {
        UIMode::Log => Style::default().fg(Color::Blue),
        _ => Style::default(),
    };
    let block = Block::default()
        .title(" Log ")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .style(style);
    frame.render_widget(block, area);

    let entries = log
        .entries
        .iter()
        .map(|line| Spans::from(Span::styled(line, Style::default().fg(Color::White))))
        .map(ListItem::new)
        .collect::<Vec<_>>();

    let list = List::new(entries)
        .block(Block::default())
        .start_corner(layout::Corner::BottomLeft);
    frame.render_widget(list, area);
}
