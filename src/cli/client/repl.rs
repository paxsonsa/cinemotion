use indiemotion_repl::{CommandResult, Value};
use std::collections::HashMap;

use super::context;
use crate::Error;
use indiemotion_proto as proto;

fn check_connection(ctx: &context::Context) -> std::result::Result<(), Error> {
    if ctx.client.is_none() {
        Err(Error::NoConnection)
    } else {
        Ok(())
    }
}

pub(crate) struct Info;

#[async_trait::async_trait]
impl indiemotion_repl::CommandHandler for Info {
    type Context = context::Context;
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
                println!("Server Info:");
                println!("  Name: {}", response.name);
                println!("  Version: {}", response.version);
                println!("  Clients:");
                for (name, client) in response.clients.iter() {
                    println!("- {}:{}", name, client.role);
                }
            }
            Err(err) => {
                tracing::error!("Failed to get server info: {}", err);
            }
        }
        Ok(CommandResult::Continue(None))
    }
}

pub(crate) struct Quit;

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

pub(crate) struct Connect;

#[async_trait::async_trait]
impl indiemotion_repl::CommandHandler for Connect {
    type Context = context::Context;
    type Error = Error;

    async fn handle(
        &mut self,
        args: HashMap<String, Value>,
        ctx: &mut Self::Context,
    ) -> std::result::Result<indiemotion_repl::CommandResult, Error> {
        ctx.connect().await?;
        Ok(CommandResult::Continue(Some("Connect".to_string())))
    }
}
