use crate::{commands::Command, Result};

use super::EngineOpt;

pub struct Engine {}

impl Engine {
    pub fn new(options: EngineOpt) -> Self {
        Self {}
    }
    pub async fn apply(&mut self, command: Command) -> Result<()> {
        Ok(())
    }
}
