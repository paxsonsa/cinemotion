mod command;
mod context;
mod handle;
mod runtime;
mod visitor;

use crate::Result;
pub use command::{CommandBuilder, CommandHandle, CommandResult, CommandType};
pub use context::{Context, ContextChannel, ContextUpdate};
pub use handle::Handle;
pub use runtime::{DefaultCommand, DefaultRuntime};
use visitor::RuntimeVisitor;

struct RuntimeBuilder<CType, Visitor>
where
    CType: CommandType,
    Visitor: RuntimeVisitor<CType> + Default + Send + 'static,
{
    _phantom: std::marker::PhantomData<(CType, Visitor)>,
}

impl<CType, Visitor> RuntimeBuilder<CType, Visitor>
where
    CType: CommandType + CommandBuilder<Command = CType>,
    Visitor: RuntimeVisitor<CType> + Default + Send + 'static,
{
    pub fn build_visitor(&self) -> Box<Visitor> {
        Box::new(Default::default())
    }

    pub async fn build(
        self,
        mut visitor: Box<Visitor>,
        mut shutdown_rx: tokio::sync::mpsc::Receiver<()>,
    ) -> Result<Handle<CType, CType>> {
        let (tx, mut rx) = tokio::sync::mpsc::channel::<CommandHandle<CType>>(1024);

        let update_channel = visitor.subscribe().await;
        let main_loop = tokio::spawn(async move {
            let mut context = Context {};
            loop {
                tokio::select! {
                    _ = shutdown_rx.recv() => {
                        tracing::info!("runtime received shutdown signal");
                        break;
                    }
                    item = rx.recv() => match item {
                        Some(handle) => {
                            tracing::debug!("received command: {:?}", handle);

                            let (command, reply) = handle.decompose();
                            match visitor.visit_command(&mut context, command).await {
                                Ok(_) => {
                                    tracing::debug!("command processed successfully");
                                    if let Err(err) = reply.send(Ok(())) {
                                        tracing::error!("reply channel closed while sending reply: {:?}", err);
                                    }
                                }
                                Err(e) => {
                                    tracing::error!("error while processing command: {:?}", e);
                                    if let Err(err) = reply.send(Err(e)) {
                                        tracing::error!("reply channel closed while sending reply: {:?}", err);
                                    }
                                }
                            };
                        }
                        None => {
                            tracing::info!("command channel closed");
                            break;
                        }
                    }
                }
            }
        });

        Ok(Handle::new(main_loop, tx, update_channel))
    }
}

pub async fn new_runtime<CType, CBuilder, Visitor>(
) -> Result<(Handle<CType, CType>, tokio::sync::mpsc::Sender<()>)>
where
    CType: CommandType + CommandBuilder<Command = CType>,
    Visitor: RuntimeVisitor<CType> + Default + Send + 'static,
{
    let builder = RuntimeBuilder::<CType, Visitor> {
        _phantom: std::marker::PhantomData,
    };
    let (shutdown_tx, shutdown_rx) = tokio::sync::mpsc::channel::<()>(1);
    let visitor = builder.build_visitor();
    let handle = builder.build(visitor, shutdown_rx).await?;
    Ok((handle, shutdown_tx))
}
