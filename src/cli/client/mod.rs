use anyhow::{Context, Result};
use clap::Args;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{borrow::BorrowMut, fmt::Debug};
use std::{io, time::Duration};
use tokio::sync::mpsc;
use tonic::transport::Uri;
use tui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};

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
        let (log_tx, log_rx) = mpsc::channel(1024);
        let log_buffer = channel_buf::ChannelBuffer::new(log_rx, 1024);
        self.configure_logging(log_tx);

        let ctx = context::Context::builder().build().await?;

        let mut state = UIState {
            mode: state::UIMode::Console,
            console: state::ConsoleState::with_repl(repl::build(ctx)),
            log: state::LogState::new(log_buffer),
        };
        let mut terminal = init()?;
        let result = run_app(&mut terminal, &mut state).await;
        let _ = shutdown(&mut terminal)?;

        if let Err(err) = result {
            return Err(err).context("Failed to run app");
        }

        Ok(0)
    }

    fn configure_logging(&self, tx: mpsc::Sender<String>) {
        let config = std::env::var("INDIEMOTION_LOG").unwrap();
        use tracing_subscriber::layer::SubscriberExt;
        let env_filter = tracing_subscriber::filter::EnvFilter::from(config);
        let registry = tracing_subscriber::Registry::default().with(env_filter);

        let layer = Box::new(state::LogLayer::new(tx));

        tracing::subscriber::set_global_default(registry.with(layer)).unwrap();
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
    // TODO: Process incoming control state into UIState.
    loop {
        let mut ui_tick = tokio::time::interval(tokio::time::Duration::from_micros(16_670));
        ui_tick.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);

        let mut ctrl_tick = tokio::time::interval(tokio::time::Duration::from_micros(16_670));
        ctrl_tick.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);

        tokio::select! {
            _ = ui_tick.tick() => {

                tracing::error!("tick tick tock");
                state.log.update().await;
                terminal.draw(|f| ui::window::render(f, state))?;

                if event::poll(Duration::from_millis(2))? {
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
                                if let InputResult::Stop = handle_console_input(state, &key).await? {
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
            _ = ctrl_tick.tick() => {
                tracing::info!("tick tick tock");
            }
        }
    }
}

enum InputResult {
    Handled,
    Stop,
}

async fn handle_console_input(state: &mut UIState, key: &event::KeyEvent) -> Result<InputResult> {
    let console = state.console.borrow_mut();
    match (key.code, key.modifiers) {
        (KeyCode::Char('c'), KeyModifiers::CONTROL) => {
            console.clear_input();
        }
        (KeyCode::Char(c), KeyModifiers::SHIFT | KeyModifiers::NONE) => {
            console.input(c);
        }
        (KeyCode::Backspace, KeyModifiers::NONE) => {
            console.backspace();
        }
        (KeyCode::Enter, KeyModifiers::NONE) => {
            // Move clear and quit into the repl and use a result type to end execution.
            if let Err(err) = console.repl.process_input().await {
                match err {
                    indiemotion_repl::Error::Stopped => {
                        return Ok(InputResult::Stop);
                    }
                    _ => return Err(err.into()),
                }
            }
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
