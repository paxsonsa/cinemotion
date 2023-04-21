use indiemotion_repl::CommandOutput;
use tui::{
    backend::Backend,
    layout::{self, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph},
    Frame,
};

use unicode_width::UnicodeWidthStr;

use super::super::state::*;

pub fn render<B: Backend>(ctx: &UIMode, console: &ConsoleState, frame: &mut Frame<B>, area: Rect) {
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

    let messages = &console
        .repl
        .output()
        .iter()
        .map(|block| {
            let (msg, style) = match &block.output {
                CommandOutput::Info(output) => {
                    (output.lines.clone(), Style::default().fg(Color::Green))
                }
                CommandOutput::Error(output) => {
                    (output.lines.clone(), Style::default().fg(Color::Red))
                }
                CommandOutput::Empty => (vec![], Style::default().fg(Color::Gray)),
            };

            let mut lines = vec![Text::styled(
                block.command.clone(),
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .fg(Color::White),
            )];
            lines.extend(
                msg.iter()
                    .map(|s| Text::styled(format!(" {}", s.clone()), style)),
            );
            lines
        })
        .flatten()
        .collect::<Vec<_>>();

    let items: Vec<ListItem> = messages
        .iter()
        .rev()
        .map(|content| ListItem::new(content.to_owned()))
        .collect();
    let list = List::new(items)
        .block(Block::default())
        .start_corner(layout::Corner::BottomLeft);
    frame.render_widget(list, sections[0]);

    let cur_input = format!(">>> {}", console.repl.current_input());
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
