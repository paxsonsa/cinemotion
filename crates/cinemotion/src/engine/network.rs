use std::collections::HashMap;

use async_trait::async_trait;

use crate::{
    commands::{event_pipe, AddConnection, Event, EventPipeTx, RequestPipeTx},
    connection::Connection,
    Error, Result,
};

use super::components::NetworkComponent;

pub struct NetworkComponentImpl {
    connections: HashMap<usize, Box<Connection>>,
    request_pipe: RequestPipeTx,
    event_pipe: EventPipeTx,
}

impl NetworkComponentImpl {
    pub fn boxed(request_pipe: RequestPipeTx) -> Box<dyn NetworkComponent> {
        let event_pipe = event_pipe();
        Box::new(Self {
            connections: Default::default(),
            request_pipe,
            event_pipe,
        })
    }
}

#[async_trait]
impl NetworkComponent for NetworkComponentImpl {
    async fn add_connection(&mut self, options: AddConnection) -> Result<()> {
        // TODO: add a connection open timeout that will close the connection if it is not opened
        // Do this on the engine side that tracks if the connection is open or not and each tick
        // check timeout.
        let agent = options.agent;
        let ack_pipe = options.ack_pipe;
        let active_id = self.connections.len() + 1;
        let conn = Box::new(Connection::new(
            active_id,
            self.request_pipe.clone(),
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

    async fn open_connection(&mut self, conn_id: usize) -> Result<()> {
        todo!()
    }

    async fn close_connection(&mut self, conn_id: usize) -> Result<()> {
        todo!()
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
