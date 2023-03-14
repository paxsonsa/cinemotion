mod command;
mod context;
mod handle;
mod runtime;
mod visitor;

use crate::Result;
pub use command::{Command, CommandHandle, CommandResult};
pub use context::{Context, ContextChannel, ContextUpdate};
pub use handle::Handle;
pub use runtime::Runtime;
use visitor::RuntimeVisitor;

pub async fn new_runtime() -> Result<(Handle, tokio::sync::mpsc::Sender<()>)> {
    let (shutdown_tx, shutdown_rx) = tokio::sync::mpsc::channel::<()>(1);
    let visitor = Box::new(Runtime::default());
    let handle = Handle::new(visitor, shutdown_rx).await;
    Ok((handle, shutdown_tx))
}
