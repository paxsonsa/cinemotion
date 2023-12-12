mod agent;
mod session;

use crate::commands::Command;
use crate::Result;

pub const LOCAL_SESSION_ID: usize = 0;
pub type SendHandlerFn = Box<dyn (FnMut(Command) -> Result<()>) + Send + Sync>;

pub use agent::SessionAgent;
pub use session::Session;
