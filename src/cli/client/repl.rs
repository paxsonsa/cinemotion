use super::context::Context;
use crate::Error;
use indiemotion_repl::*;
use std::collections::HashMap;

use indiemotion_proto as proto;

pub type Repl = indiemotion_repl::Repl<Context, crate::Error>;

pub fn build(context: Context) -> Repl {
    let mut repl = Repl::new(
        env!("CARGO_PKG_NAME").to_string(),
        env!("CARGO_PKG_VERSION").to_string(),
        env!("CARGO_PKG_DESCRIPTION").to_string(),
        context,
    );
    repl = repl.with_command(Name::command());
    repl = repl.with_command(Quit::command());
    repl = repl.with_command(Info::command());
    repl = repl.with_command(Connect::command());

    // repl.add_command("name", Name);
    // repl.add_command("role", Role);
    // repl.add_command("info", Info);
    // repl.add_command("connect", Connect);
    // repl.add_command("disconnect", Disconnect);
    // repl.add_command("list", List);
    // repl.add_command("add", Add);
    // repl.add_command("remove", Remove);
    // repl.add_command("start", Start);
    // repl.add_command("stop", Stop);
    // repl.add_command("exit", Exit);
    repl
}

fn check_connection(ctx: &Context) -> std::result::Result<(), Error> {
    if ctx.client.is_none() {
        Err(Error::NoConnection)
    } else {
        Ok(())
    }
}

struct Name;

impl Name {
    fn command() -> Command<Context, crate::Error> {
        Command::new("name", Box::new(Self))
            .with_parameter(Parameter::new("name"))
            .unwrap()
            .with_help("Get or set the name of the client.")
    }
}

#[async_trait::async_trait]
impl indiemotion_repl::CommandHandler for Name {
    type Context = Context;
    type Error = Error;

    async fn handle(
        &mut self,
        args: HashMap<String, Value>,
        ctx: &mut Self::Context,
    ) -> std::result::Result<indiemotion_repl::CommandResult, Error> {
        if let Some(name) = args.get("name") {
            ctx.name = name.convert()?;
        }
        Ok(CommandResult::Output(CommandOutput::Info(BlockOutput {
            lines: vec![ctx.name.clone()],
        })))
    }
}

struct Quit;

impl Quit {
    fn command() -> Command<Context, crate::Error> {
        Command::new("quit", Box::new(Self)).with_help("quit the application.")
    }
}

#[async_trait::async_trait]
impl indiemotion_repl::CommandHandler for Quit {
    type Context = Context;
    type Error = Error;

    async fn handle(
        &mut self,
        _args: HashMap<String, Value>,
        _ctx: &mut Self::Context,
    ) -> std::result::Result<indiemotion_repl::CommandResult, Error> {
        Ok(CommandResult::Stop)
    }
}

pub(crate) struct Connect;

impl Connect {
    fn command() -> Command<Context, crate::Error> {
        Command::new("connect", Box::new(Self)).with_help("connect to the specified addr")
    }
}

#[async_trait::async_trait]
impl indiemotion_repl::CommandHandler for Connect {
    type Context = Context;
    type Error = Error;

    async fn handle(
        &mut self,
        _args: HashMap<String, Value>,
        ctx: &mut Self::Context,
    ) -> std::result::Result<indiemotion_repl::CommandResult, Error> {
        ctx.connect().await?;

        let info = proto::ClientInfo {
            id: ctx.uid.clone().to_string(),
            name: ctx.name.clone(),
            role: Into::<proto::ClientRole>::into(ctx.role.clone()).into(),
        };

        let req = proto::ConnectAsRequest {
            client_info: Some(info),
        };

        println!("Connecting to server...");
        match ctx.client.as_mut().unwrap().connect_as(req).await {
            Ok(response) => {
                //TODO Write to log bus
                // println!("Establishing loop...");
                let mut stream = response.into_inner();
                ctx.main_loop = Some(tokio::spawn(async move {
                    while let Some(_event) = stream.message().await.unwrap() {
                        // TODO: Write to log bus.
                    }
                }));
                Ok(CommandResult::Output(CommandOutput::info(
                    "connected".to_string(),
                )))
            }
            Err(_) => Ok(CommandResult::Output(CommandOutput::error(
                "failed to connect".to_string(),
            ))),
        }
    }
}

/// Command for extracting the information from a server.
pub(crate) struct Info;

impl Info {
    fn command() -> Command<Context, crate::Error> {
        Command::new("info", Box::new(Self))
            .with_help("request info from the connected server or specified addr.")
    }
}

#[async_trait::async_trait]
impl indiemotion_repl::CommandHandler for Info {
    type Context = Context;
    type Error = Error;

    async fn handle(
        &mut self,
        _args: HashMap<String, Value>,
        ctx: &mut Self::Context,
    ) -> std::result::Result<indiemotion_repl::CommandResult, Error> {
        check_connection(ctx)?;
        let request = proto::ServerInfoRequest {};
        match ctx.client.as_mut().unwrap().server_info(request).await {
            Ok(response) => {
                let response = response.into_inner();
                let mut block = BlockOutput::default();
                block.add_line("Server Info:");
                block.add_line(format!("  Name: {}", response.name));
                block.add_line(format!("  Version: {}", response.version));
                block.add_line(format!("  Clients:"));
                for (name, client) in response.clients.iter() {
                    block.add_line(format!("- {}:{}", name, client.role));
                }
                block.add_line(format!("  ctx: {:?}", response));
                Ok(CommandResult::Output(CommandOutput::Info(block)))
            }
            Err(err) => Err(Error::CommandFailed(format!(
                "Failed to get server info: {}",
                err
            ))),
        }
    }
}
