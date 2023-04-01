use crate::error::*;
use crate::Parameter;
use crate::Value;
use std::fmt;

use std::collections::HashMap;

pub enum CommandResult {
    Continue(Option<String>),
    Stop,
}

/// Command Handler
#[async_trait::async_trait]
pub trait CommandHandler {
    type Context;
    type Error;
    async fn handle(
        &mut self,
        args: HashMap<String, Value>,
        ctx: &mut Self::Context,
    ) -> std::result::Result<CommandResult, Self::Error>;
}

/// Struct to define a command in the REPL
pub struct Command<C, E> {
    pub(crate) name: String,
    pub(crate) parameters: Vec<Parameter>,
    pub(crate) handler: Box<dyn CommandHandler<Context = C, Error = E>>,
    pub(crate) help_summary: Option<String>,
}

impl<C, E> fmt::Debug for Command<C, E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Command")
            .field("name", &self.name)
            .field("parameters", &self.parameters)
            .field("help_summary", &self.help_summary)
            .finish()
    }
}

impl<C, E> std::cmp::PartialEq for Command<C, E> {
    fn eq(&self, other: &Command<C, E>) -> bool {
        self.name == other.name
            && self.parameters == other.parameters
            && self.help_summary == other.help_summary
    }
}

impl<C, E> Command<C, E> {
    /// Create a new command with the given name and callback function
    pub fn new(name: &str, handler: Box<dyn CommandHandler<Context = C, Error = E>>) -> Self {
        Self {
            name: name.to_string(),
            parameters: vec![],
            handler,
            help_summary: None,
        }
    }

    /// Add a parameter to the command. The order of the parameters is the same as the order in
    /// which this is called for each parameter.
    pub fn with_parameter(mut self, parameter: Parameter) -> Result<Command<C, E>> {
        if parameter.required && self.parameters.iter().any(|param| !param.required) {
            return Err(Error::IllegalRequiredError(parameter.name));
        }

        self.parameters.push(parameter);

        Ok(self)
    }

    /// Add a help summary for the command
    pub fn with_help(mut self, help: &str) -> Command<C, E> {
        self.help_summary = Some(help.to_string());

        self
    }
}