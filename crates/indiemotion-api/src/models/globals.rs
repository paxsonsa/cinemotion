use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
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
