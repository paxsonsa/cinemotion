use std::collections::HashMap;

use async_trait::async_trait;

use crate::{
    connection::{Connection, Context},
    events::{event_pipe, EventPipeTx},
    messages::{AddConnection, MessagePipeTx},
    Error, Event, Result,
};

use super::components::network::NetworkComponent;

pub struct NetworkComponentImpl {
    connections: HashMap<usize, Box<Connection>>,
    contexts: HashMap<usize, Context>,
    message_pipe: MessagePipeTx,
    event_pipe: EventPipeTx,
}

impl NetworkComponentImpl {
    pub fn boxed(message_pipe: MessagePipeTx) -> Box<dyn NetworkComponent> {
        let event_pipe = event_pipe();
        Box::new(Self {
            connections: Default::default(),
            contexts: Default::default(),
            message_pipe,
            event_pipe,
        })
    }
}

#[async_trait]
impl NetworkComponent for NetworkComponentImpl {
    /// Get the context for a connection id.
    fn context(&self, conn_id: usize) -> Option<&Context> {
        match self.contexts.get(&conn_id) {
            Some(ctx) => Some(ctx),
            None => None,
        }
    }

    /// Get a mutable reference to the context for a connection id.
    /// If the context does not exist then create it.
    fn context_mut(&mut self, conn_id: usize) -> &mut Context {
        self.contexts.entry(conn_id).or_default();
        self.contexts.get_mut(&conn_id).unwrap()
    }

    async fn add_connection(&mut self, options: AddConnection) -> Result<()> {
        // TODO: add a connection open timeout that will close the connection if it is not opened
        // Do this on the engine side that tracks if the connection is open or not and each tick
        // check timeout.
        let agent = options.agent;
        let ack_pipe = options.ack_pipe;
        // FIXME: change this to use a atomic incrementor, if connection disconnect and the
        // reconnect then the id migh be reused.
        let active_id = self.connections.len() + 1;
        let conn = Box::new(Connection::new(
            active_id,
            self.message_pipe.clone(),
            self.event_pipe.subscribe(),
            agent,
        ));
        self.connections.insert(active_id, conn);
        if ack_pipe.send(Ok(active_id)).is_err() {
            tracing::error!(
                "create ack pipe dropped while creating connection, dropping connection"
            );
            let _ = self.connections.remove(&active_id);
        }
        Ok(())
    }

    async fn close_connection(&mut self, conn_id: usize) -> Result<()> {
        self.connections.remove(&conn_id);
        self.contexts.remove(&conn_id);
        Ok(())
    }

    async fn send(&mut self, event: Event) -> Result<()> {
        if self.event_pipe.send(event).is_err() {
            return Err(Error::EngineFailed(
                "event pipe closed unexpectedly.".to_string(),
            ));
        }
        Ok(())
    }
}
