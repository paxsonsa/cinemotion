use super::{ClientCommand, Payload, SystemCommand};
use crate::connection::ConnectionAgent;
use crate::data::controllers;
use crate::Result;
use cinemotion_proto as proto;
use std::fmt::Debug;

#[derive(Debug)]
pub struct Init {
    pub peer: controllers::Controller,
}

impl From<Init> for Payload {
    fn from(value: Init) -> Self {
        Self::Client(ClientCommand::Init(value))
    }
}

impl From<proto::InitCommand> for Init {
    fn from(value: proto::InitCommand) -> Self {
        Self {
            peer: value.controller.unwrap().into(),
        }
    }
}

#[derive(Debug)]
pub struct OpenConnection {}

impl From<OpenConnection> for Payload {
    fn from(value: OpenConnection) -> Self {
        Self::System(SystemCommand::OpenConnection(value))
    }
}

pub struct AddConnection {
    pub agent: Box<dyn ConnectionAgent + Send + Sync>,
    pub ack_pipe: tokio::sync::oneshot::Sender<Result<usize>>,
}

impl Debug for AddConnection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AddConnection")
            .field("agent", &"Box<dyn ConnectionAgent + Send + Sync>")
            .field("ack_pipe", &"tokio::sync::oneshot::Sender<Result<usize>>")
            .finish()
    }
}

impl From<AddConnection> for Payload {
    fn from(value: AddConnection) -> Self {
        Self::System(SystemCommand::AddConnection(value))
    }
}

#[derive(Debug)]
pub struct CloseConnection {}

impl From<CloseConnection> for Payload {
    fn from(value: CloseConnection) -> Self {
        Self::System(SystemCommand::CloseConnection(value))
    }
}
