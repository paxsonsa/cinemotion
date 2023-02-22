#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SessionMode {
    Idle,
    Live,
    Recording, // TODO add track id when recording.
}

impl SessionMode {
    pub fn variant_eq(&self, mode: &SessionMode) -> bool {
        match (self, mode) {
            (SessionMode::Idle, SessionMode::Idle) => true,
            (SessionMode::Live, SessionMode::Live) => true,
            (SessionMode::Recording, SessionMode::Recording) => true,
            _ => false
        }
    }
}

pub struct SessionState {
    pub mode: SessionMode
}

impl Default for SessionState {
    fn default() -> Self {
        Self {
            mode: SessionMode::Idle
        }
    }
}
