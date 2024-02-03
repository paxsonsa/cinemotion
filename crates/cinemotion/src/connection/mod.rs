mod agent;
mod connection;
mod context;

use crate::messages::Payload;
use crate::Result;

pub const LOCAL_CONN_ID: usize = 0;
pub type SendHandlerFn = Box<dyn (FnMut(Payload) -> Result<()>) + Send + Sync>;

pub use agent::*;
pub use connection::*;
pub use context::*;
