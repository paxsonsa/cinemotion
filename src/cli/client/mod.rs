use anyhow::Result;
use clap::Args;

use indiemotion_repl::{Command, CommandResult, Parameter, Repl, Value};
use std::{collections::HashMap, fmt::Display};
use tonic::transport::Uri;

mod context;
mod repl;

/// Example using Repl with a custom prompt
struct Prompt;

impl Display for Prompt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, ">>> ")
    }
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

        let info_handler = Box::new(repl::Info {});
        let quit_handler = Box::new(repl::Quit {});
        let connect_handler = Box::new(repl::Connect {});
        let name_handler = Box::new(repl::Name {});
        let role_handler = Box::new(repl::Role {});

        let mut repl = Repl::new(ctx)
            .with_name("IndieMotion Debug Client")
            .with_version(format!("v{}", indiemotion::VERSION).as_str())
            .with_description("A command line client for IndieMotion used for Debugging, Monitoring, and Testing.")
            .with_prompt(&Prompt)
            .use_completion(true)
            .add_command(
                Command::new("quit", quit_handler)
                    .with_help("Quit the client session."),
            )
            .add_command(
                Command::new("connect", connect_handler)
                    .with_parameter(Parameter::new("address"))?
                    .with_help("Connect to a server instance."),
            )
            .add_command(
                Command::new("info", info_handler)
                    .with_help("Print info about the server."),
            )
            .add_command(
                Command::new("name", name_handler)
                    .with_help("Set or print the name of the session.")
                    .with_parameter(Parameter::new("name"))?,
            ).add_command(
                Command::new("role", role_handler)
                    .with_help("Set or print the client role. (Valid roles are: 'primary', 'secondary', 'observer')")
                    .with_parameter(Parameter::new("role"))?,
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
