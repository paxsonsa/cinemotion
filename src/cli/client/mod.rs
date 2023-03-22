use anyhow::{Context, Result};
use clap::Args;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::fmt::Debug;
use std::io;
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{self, Alignment, Constraint, Direction, Layout, Rect},
    style::{self, Color, Modifier, Style},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph, Wrap},
    Frame, Terminal,
};
// use unicode_width::UnicodeWidthStr;

use indiemotion_repl::{Command, Parameter, Repl};
use std::fmt::Display;
use tonic::transport::Uri;

mod context;
mod repl;
mod state;
mod ui;

use state::UIState;

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
        let mut state = UIState::default();
        let mut terminal = init()?;
        let result = run_app(&mut terminal, &mut state).await;
        let _ = shutdown(&mut terminal)?;

        if let Err(err) = result {
            return Err(err).context("Failed to run app");
        }

        Ok(0)
    }
}

fn init() -> Result<Terminal<CrosstermBackend<std::io::Stdout>>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    Ok(Terminal::new(backend)?)
}

async fn run_app<B: Backend>(terminal: &mut Terminal<B>, state: &mut UIState) -> Result<()> {
    loop {
        terminal.draw(|f| ui::window::render(f, state))?;

        if let Event::Key(key) = event::read()? {
            match (key.code, key.modifiers) {
                (KeyCode::Char('c'), KeyModifiers::CONTROL) => {
                    state.console.clear_input();
                }
                (KeyCode::Char('d'), KeyModifiers::CONTROL) => {
                    return Ok(());
                }
                (KeyCode::Char(c), KeyModifiers::SHIFT | KeyModifiers::NONE) => {
                    state.console.input(c);
                }
                (KeyCode::Backspace, KeyModifiers::NONE) => {
                    state.console.backspace();
                }
                (KeyCode::Tab, _) => {
                    state.mode = state.mode.cycle();
                }
                (KeyCode::BackTab, _) => {
                    state.mode = state.mode.cycle_back();
                }
                (KeyCode::Enter, KeyModifiers::NONE) => {
                    state.console.push_history();
                    state.console.clear_input();
                }
                (KeyCode::Up, KeyModifiers::NONE) => {
                    state.console.history_up();
                }
                (KeyCode::Down, KeyModifiers::NONE) => {
                    state.console.history_down();
                }
                _ => {}
            }
        }
    }
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
