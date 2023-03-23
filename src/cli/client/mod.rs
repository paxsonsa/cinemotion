use anyhow::{Context, Result};
use clap::Args;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::collections::HashMap;
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

// use indiemotion_repl::{Command, Parameter, Repl};
use std::fmt::Display;
use tonic::transport::Uri;

mod context;
mod repl;
mod state;
mod ui;

use state::UIState;

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
        let mut repl = repl::Repl::new();
        let result = run_app(&mut terminal, &mut repl, &mut state).await;
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

async fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    repl: &mut repl::Repl,
    state: &mut UIState,
) -> Result<()> {
    loop {
        terminal.draw(|f| ui::window::render(f, state))?;

        if let Event::Key(key) = event::read()? {
            match (&key.code, &key.modifiers) {
                (KeyCode::Char('d'), &KeyModifiers::CONTROL) => {
                    return Ok(());
                }
                (KeyCode::Tab, _) => {
                    state.mode = state.mode.cycle();
                }
                (KeyCode::BackTab, _) => {
                    state.mode = state.mode.cycle_back();
                }
                (_, _) => {}
            };

            match state.mode {
                state::UIMode::Console => {
                    if let InputResult::Stop = handle_console_input(state, repl, &key).await? {
                        return Ok(());
                    }
                }
                state::UIMode::Outliner => {
                    if let InputResult::Stop = handle_outliner_input(state, &key).await? {
                        return Ok(());
                    }
                }
                state::UIMode::Log => {
                    if let InputResult::Stop = handle_log_input(state, &key).await? {
                        return Ok(());
                    }
                }
            }
        }
    }
}

enum InputResult {
    Handled,
    Stop,
}

async fn handle_console_input(
    state: &mut UIState,
    repl: &mut repl::Repl,
    key: &event::KeyEvent,
) -> Result<InputResult> {
    match (key.code, key.modifiers) {
        (KeyCode::Char('c'), KeyModifiers::CONTROL) => {
            state.console.clear_input();
        }
        (KeyCode::Char(c), KeyModifiers::SHIFT | KeyModifiers::NONE) => {
            state.console.input(c);
        }
        (KeyCode::Backspace, KeyModifiers::NONE) => {
            state.console.backspace();
        }
        (KeyCode::Enter, KeyModifiers::NONE) => {
            state.console.push_history();
            let input = state.console.cur_input.clone();
            let input = input.trim();
            match input {
                "quit" => {
                    return Ok(InputResult::Stop);
                }
                "clear" => {
                    state.console.messages.clear();
                }
                input => match repl.readline(input.to_string()).await {
                    Ok(output) => {
                        if !output.is_empty() {
                            state.console.messages.push(output);
                        }
                    }
                    Err(err) => {
                        state.console.messages.push(format!("Error: {}", err));
                    }
                },
            }
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
    Ok(InputResult::Handled)
}

async fn handle_outliner_input(state: &mut UIState, key: &event::KeyEvent) -> Result<InputResult> {
    Ok(InputResult::Handled)
}

async fn handle_log_input(state: &mut UIState, key: &event::KeyEvent) -> Result<InputResult> {
    Ok(InputResult::Handled)
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
