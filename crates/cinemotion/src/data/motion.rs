#[derive(Clone, Copy, Debug, PartialEq)]
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
