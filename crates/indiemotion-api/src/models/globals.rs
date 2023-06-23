use serde::{Deserialize, Serialize};

#[cfg(test)]
#[path = "./globals_test.rs"]
mod globals_test;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[serde(tag = "mode")]
pub enum Mode {
    Idle,
    Live,
    Recording,
}

impl Default for Mode {
    fn default() -> Self {
        Self::Idle
    }
}

impl Mode {
    pub fn is_idle(&self) -> bool {
        matches!(self, Mode::Idle)
    }

    pub fn is_live(&self) -> bool {
        matches!(self, Mode::Live | Mode::Recording)
    }

    pub fn is_recording(&self) -> bool {
        matches!(self, Mode::Recording)
    }
}
