use indiemotion_repl::{CommandResult, Convert, Value};
use std::collections::HashMap;

use super::context;
use crate::Error;
use indiemotion_api as api;
use indiemotion_proto as proto;

fn check_connection(ctx: &context::Context) -> std::result::Result<(), Error> {
    if ctx.client.is_none() {
        Err(Error::NoConnection)
    } else {
        Ok(())
    }
}

pub(crate) struct Name;

#[async_trait::async_trait]
impl indiemotion_repl::CommandHandler for Name {
    type Context = context::Context;
    type Error = Error;

    async fn handle(
        &mut self,
        args: HashMap<String, Value>,
        ctx: &mut Self::Context,
    ) -> std::result::Result<indiemotion_repl::CommandResult, Error> {
        if let Some(name) = args.get("name") {
            ctx.name = name.convert()?;
        }
        Ok(CommandResult::Continue(Some(ctx.name.clone())))
    }
}

pub(crate) struct Role;

#[async_trait::async_trait]
impl indiemotion_repl::CommandHandler for Role {
    type Context = context::Context;
    type Error = Error;

    async fn handle(
        &mut self,
        args: HashMap<String, Value>,
        ctx: &mut Self::Context,
    ) -> std::result::Result<indiemotion_repl::CommandResult, Error> {
        if let Some(role) = args.get("role") {
            let s: String = role.convert()?;
            ctx.role = s.try_into()?;
        }
        Ok(CommandResult::Continue(Some(ctx.role.clone().into())))
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
                println!("Establishing loop...");
                let mut stream = response.into_inner();
                ctx.main_loop = Some(tokio::spawn(async move {
                    while let Some(event) = stream.message().await.unwrap() {
                        println!("Event: {:?}", event);
                    }
                }));
                println!("Connected to server");
            }
            Err(err) => {
                tracing::error!("Failed to connect: {}", err);
            }
        }
        Ok(CommandResult::Continue(Some("Connect".to_string())))
    }
}
