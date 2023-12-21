mod agent;
mod connection;

use crate::commands::Command;
use crate::Result;

pub const LOCAL_CONN_ID: usize = 0;
pub type SendHandlerFn = Box<dyn (FnMut(Command) -> Result<()>) + Send + Sync>;

pub use agent::ConnectionAgent;
pub use connection::Connection;
