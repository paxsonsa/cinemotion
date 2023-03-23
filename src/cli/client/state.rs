use std::fmt::Debug;

#[derive(Debug, Default)]
pub struct UIState {
    pub mode: UIMode,
    pub console: ConsoleState,
}

#[derive(Debug, Default)]
pub struct ConsoleState {
    pub cur_input: String,
    pub messages: Vec<String>,
    pub history: Vec<String>,
    pub history_index: usize,
}

impl ConsoleState {
    pub fn input(&mut self, ch: char) {
        self.history_index = self.history.len();
        self.cur_input.push(ch);
    }

    pub fn backspace(&mut self) {
        self.cur_input.pop();
    }

    pub fn clear_input(&mut self) {
        self.history_index = self.history.len();
        self.cur_input.clear();
    }

    pub fn push_history(&mut self) {
        self.messages.push(self.cur_input.clone());
        self.history.push(self.cur_input.clone());
        self.history_index = self.history.len();
    }

    pub fn history_up(&mut self) {
        if self.history_index == 0 {
            self.cur_input = self.history[self.history_index].clone();
            return;
        }
        self.history_index = self.history_index - 1;
        self.cur_input = self.history[self.history_index].clone();
    }

    pub fn history_down(&mut self) {
        let index = self.history_index + 1;
        if index >= self.history.len() {
            self.history_index = self.history.len();
            self.cur_input.clear();
            return;
        }

        self.history_index = index;
        self.cur_input = self.history[self.history_index].clone();
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
