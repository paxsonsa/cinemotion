use anyhow::Result;
use clap::Args;
use thiserror::Error;

use indiemotion_repl::{Command, CommandHandler, CommandResult, Parameter, Repl, Value};
use std::collections::HashMap;
use tonic::transport::Uri;

mod context;

#[derive(Error, Debug)]
enum Error {
    #[error("Must be connected to a server to use this command.")]
    NoConnection,

    #[error("Ending Session")]
    Quit,

    #[error("An error occurred while interacting with the REPL.")]
    ReplError(indiemotion_repl::Error),
}

impl From<indiemotion_repl::Error> for Error {
    fn from(error: indiemotion_repl::Error) -> Self {
        Error::ReplError(error)
    }
}

fn check_connection(ctx: &context::Context) -> std::result::Result<(), Error> {
    if ctx.client.is_none() {
        Err(Error::NoConnection)
    } else {
        Ok(())
    }
}

struct Info;

#[async_trait::async_trait]
impl indiemotion_repl::CommandHandler for Info {
    type Context = context::Context;
    type Error = Error;

    async fn handle(
        &mut self,
        args: HashMap<String, Value>,
        ctx: &mut Self::Context,
    ) -> std::result::Result<indiemotion_repl::CommandResult, Error> {
        check_connection(ctx)?;
        Ok(CommandResult::Continue(Some("Info".to_string())))
    }
}

struct Quit;

#[async_trait::async_trait]
impl indiemotion_repl::CommandHandler for Quit {
    type Context = context::Context;
    type Error = Error;

    async fn handle(
        &mut self,
        _args: HashMap<String, Value>,
        _ctx: &mut Self::Context,
    ) -> std::result::Result<indiemotion_repl::CommandResult, Error> {
        Ok(CommandResult::Stop)
    }
}

struct Connect;

#[async_trait::async_trait]
impl indiemotion_repl::CommandHandler for Connect {
    type Context = context::Context;
    type Error = Error;

    async fn handle(
        &mut self,
        args: HashMap<String, Value>,
        ctx: &mut Self::Context,
    ) -> std::result::Result<indiemotion_repl::CommandResult, Error> {
        Ok(CommandResult::Continue(Some("Connect".to_string())))
    }
}

// Print Server Info
fn handle_info(_: HashMap<String, Value>, ctx: &mut context::Context) -> Result<Option<String>> {
    check_connection(ctx)?;
    Ok(Some("Info".to_string()))
}

// Connect to Server at Address.
async fn handle_connect(
    _: HashMap<String, Value>,
    ctx: &mut context::Context,
) -> Result<Option<String>> {
    ctx.connect().await?;
    Ok(Some(format!("connected: {}", ctx.address.clone().unwrap())))
}

// Quit Server
fn handle_quit(_: HashMap<String, Value>, ctx: &mut context::Context) -> Result<Option<String>> {
    check_connection(ctx)?;
    Ok(Some("Quit".to_string()))
}

#[derive(Args, Debug)]
pub struct Client {
    /// The address and port to connect to the server on.
    #[clap(long = "addr")]
    address: Option<Uri>,
}

impl Client {
    pub async fn run(&self) -> Result<i32> {
        let mut builder = context::Context::builder();
        if let Some(addr) = self.address.clone() {
            builder = builder.with_addr(addr);
        }
        let ctx = builder.build().await?;

        let info_handler = Box::new(Info {});
        let quit_handler = Box::new(Quit {});
        let connect_handler = Box::new(Connect {});

        let mut repl = Repl::new(ctx)
            .with_name("IndieMotion Debug Client")
            .with_version(format!("v{}", indiemotion::VERSION).as_str())
            .with_description("A command line client for IndieMotion used for Debugging, Monitoring, and Testing.")
            .add_command(
                Command::new("quit", quit_handler)
                    .with_help("Quit the client session."),
            )
            .add_command(
                Command::new("connect", connect_handler)
                    .with_help("Connect to a server instance."),
            )
            .add_command(
                Command::new("info", info_handler)
                    .with_help("Print info about the server."),
            );

        match repl.run().await {
            Ok(_) => Ok(0),
            Err(err) => {
                tracing::error!(?err, "Client encountered and error");
                Ok(1)
            }
        }
    }
}
