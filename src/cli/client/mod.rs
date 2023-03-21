use anyhow::{Context, Result};
use clap::Args;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    error::Error,
    fmt::Debug,
    fs::File,
    io::{self, Write},
    os::fd::{AsFd, AsRawFd, FromRawFd},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{self, Alignment, Constraint, Direction, Layout, Rect},
    style::{self, Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph, Wrap},
    Frame, Terminal,
};
// use unicode_width::UnicodeWidthStr;

use indiemotion_repl::{Command, Parameter, Repl};
use std::fmt::Display;
use tonic::transport::Uri;

mod context;
mod repl;

/// Example using Repl with a custom prompt
struct Prompt;

impl Display for Prompt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, ">>> ")
    }
}

#[derive(Args, Debug)]
pub struct Client {
    /// The address and port to connect to the server on.
    #[clap(long = "addr")]
    address: Option<Uri>,
}

impl Client {
    pub async fn run(&self) -> Result<i32> {
        let mut app = AppState::default();
        let mut terminal = init()?;
        let result = run_app(&mut terminal, &mut app).await;
        let _ = shutdown(&mut terminal)?;

        if let Err(err) = result {
            return Err(err).context("Failed to run app");
        }

        Ok(0)
    }
}

enum UIContext {
    Console,
    Outliner,
    Log,
}

impl UIContext {
    fn cycle(&self) -> Self {
        match self {
            UIContext::Console => UIContext::Outliner,
            UIContext::Outliner => UIContext::Log,
            UIContext::Log => UIContext::Console,
        }
    }

    fn cycle_back(&self) -> Self {
        match self {
            UIContext::Console => UIContext::Log,
            UIContext::Log => UIContext::Outliner,
            UIContext::Outliner => UIContext::Console,
        }
    }
}

impl Default for UIContext {
    fn default() -> Self {
        UIContext::Console
    }
}

impl Debug for UIContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UIContext::Console => write!(f, "Console"),
            UIContext::Outliner => write!(f, "Outliner"),
            UIContext::Log => write!(f, "Log"),
        }
    }
}

#[derive(Debug, Default)]
struct AppState {
    input: String,
    messages: Vec<String>,
    history: Vec<String>,
    ctx: UIContext,
}

fn init() -> Result<Terminal<CrosstermBackend<std::io::Stdout>>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    Ok(Terminal::new(backend)?)
}

async fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut AppState) -> Result<()> {
    loop {
        terminal.draw(|f| draw_ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            match (key.code, key.modifiers) {
                (KeyCode::Esc, KeyModifiers::NONE)
                | (KeyCode::Char('c'), KeyModifiers::CONTROL) => {
                    app.input.clear();
                }
                (KeyCode::Char('d'), KeyModifiers::CONTROL) => {
                    return Ok(());
                }
                (KeyCode::Char(c), KeyModifiers::SHIFT | KeyModifiers::NONE) => {
                    app.input.push(c);
                }
                (KeyCode::Backspace, KeyModifiers::NONE) => {
                    app.input.pop();
                }
                (KeyCode::Tab, _) => {
                    app.ctx = app.ctx.cycle();
                }
                (KeyCode::BackTab, _) => {
                    app.ctx = app.ctx.cycle_back();
                }
                (KeyCode::Enter, KeyModifiers::NONE) => {
                    app.messages.push(app.input.clone());
                    app.input.clear();
                }
                _ => {}
            }
        }
    }
}

fn draw_ui<B: Backend>(frame: &mut Frame<B>, app: &mut AppState) {
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

    draw_console(&app.ctx, &app.input, &app.messages, frame, console);

    let style = match app.ctx {
        UIContext::Log => Style::default().fg(Color::Blue),
        _ => Style::default(),
    };
    let block = Block::default()
        .title(" Log ")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .style(style);
    frame.render_widget(block, log);

    let style = match app.ctx {
        UIContext::Outliner => Style::default().fg(Color::Blue),
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

fn draw_console<B: Backend>(
    ctx: &UIContext,
    input: &String,
    messages: &Vec<String>,
    frame: &mut Frame<B>,
    area: Rect,
) {
    let style = match ctx {
        UIContext::Console => Style::default().fg(Color::Blue),
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

    let cur_input = format!("> {}", input);
    let input = Paragraph::new(cur_input).style(
        Style::default()
            .fg(Color::White)
            .add_modifier(Modifier::BOLD),
    );
    frame.render_widget(input, sections[1]);
}

fn draw_log<B: Backend>(frame: &mut Frame<B>, area: Rect) {
    let block = Block::default()
        .title(" Log ")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .style(Style::default().fg(Color::White));
    frame.render_widget(block, area);
}

fn shutdown(terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>) -> Result<()> {
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}
