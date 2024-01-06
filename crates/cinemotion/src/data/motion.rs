use cinemotion_proto::proto;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Mode {
    Idle,
    Live,
    Recording,
}

impl Mode {
    /// Returns true if mode is idle
    pub fn is_idle(&self) -> bool {
        *self == Self::Idle
    }
    /// Returns true if mode is live (or recording)
    pub fn is_live(&self) -> bool {
        *self == Self::Live
    }
    /// Returns true if mode is recording
    pub fn is_recording(&self) -> bool {
        *self == Self::Recording
    }
}

impl From<proto::change_mode::Mode> for Mode {
    fn from(value: proto::change_mode::Mode) -> Self {
        match value {
            proto::change_mode::Mode::Idle => Self::Idle,
            proto::change_mode::Mode::Live => Self::Live,
            proto::change_mode::Mode::Recording => Self::Recording,
        }
    }
}

impl Default for Mode {
    fn default() -> Self {
        Self::Idle
    }
}
