mod command;
mod context;
mod handle;
pub use command::{Command, CommandHandle};
pub use context::{Context, Event};
pub use handle::Handle;

use crate::Result;

#[async_trait::async_trait]
pub trait Integrator {
    async fn tick(&mut self, context: &mut Context) -> Result<()>;
    async fn process(&mut self, context: &mut Context, command: Command) -> Result<()>;
}
