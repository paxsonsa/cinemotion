use std::fmt::Debug;

use super::repl;

#[derive(Debug)]
pub struct UIState {
    pub mode: UIMode,
    pub console: ConsoleState,
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
