use std::fmt;
use std::fmt::Debug;
use tokio::sync::mpsc;
use tracing_subscriber::Layer;

use super::repl;

#[derive(Debug)]
pub struct UIState {
    pub mode: UIMode,
    pub console: ConsoleState,
    pub log: LogState,
}

pub struct ConsoleState {
    pub repl: repl::Repl,
}

impl std::fmt::Debug for ConsoleState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ConsoleState").finish()
    }
}

impl ConsoleState {
    pub fn with_repl(repl: repl::Repl) -> Self {
        ConsoleState { repl }
    }

    pub fn input(&mut self, ch: char) {
        self.repl.push(ch);
    }

    pub fn backspace(&mut self) {
        self.repl.pop();
    }

    pub fn clear_input(&mut self) {
        self.repl.clear_input();
    }

    pub fn history_up(&mut self) {
        self.repl.history_up();
    }

    pub fn history_down(&mut self) {
        self.repl.history_down();
    }
}

pub struct LogState {
    buffer: channel_buf::ChannelBuffer<String>,
    pub entries: Vec<String>,
}

impl fmt::Debug for LogState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("LogState").finish()
    }
}
impl LogState {
    pub fn new(buffer: channel_buf::ChannelBuffer<String>) -> Self {
        LogState {
            buffer,
            entries: vec![],
        }
    }

    pub async fn update(&mut self) {
        self.entries.extend(self.buffer.flush().await);
    }
}

pub enum UIMode {
    Console,
    Outliner,
    Log,
}

impl UIMode {
    pub fn cycle(&self) -> Self {
        match self {
            UIMode::Console => UIMode::Outliner,
            UIMode::Outliner => UIMode::Log,
            UIMode::Log => UIMode::Console,
        }
    }

    pub fn cycle_back(&self) -> Self {
        match self {
            UIMode::Console => UIMode::Log,
            UIMode::Log => UIMode::Outliner,
            UIMode::Outliner => UIMode::Console,
        }
    }
}

impl Default for UIMode {
    fn default() -> Self {
        UIMode::Console
    }
}

impl Debug for UIMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UIMode::Console => write!(f, "Console"),
            UIMode::Outliner => write!(f, "Outliner"),
            UIMode::Log => write!(f, "Log"),
        }
    }
}

#[derive(Debug)]
pub struct LogLayer {
    channel: mpsc::Sender<String>,
}

impl LogLayer {
    pub fn new(channel: mpsc::Sender<String>) -> Self {
        LogLayer { channel }
    }
}

impl<S> Layer<S> for LogLayer
where
    S: tracing::Subscriber,
{
    fn on_event(
        &self,
        event: &tracing::Event<'_>,
        _ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        // TODO: Visit fields and log them
        let mut msg = format!(
            "{:?} target={:?} name={:?} fields=",
            event.metadata().level(),
            event.metadata().target(),
            event.metadata().name()
        );
        for field in event.fields() {
            msg += &format!("{},", field.name());
        }

        self.channel.try_send(msg).unwrap();
    }
}
