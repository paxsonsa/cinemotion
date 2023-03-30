use super::context::Context;
use crate::Error;
use indiemotion_repl::*;
use std::collections::HashMap;

pub type Repl = indiemotion_repl::Repl<Context, crate::Error>;

pub fn build(context: Context) -> Repl {
    let mut repl = Repl::new(
        env!("CARGO_PKG_NAME").to_string(),
        env!("CARGO_PKG_VERSION").to_string(),
        env!("CARGO_PKG_DESCRIPTION").to_string(),
        context,
    );
    repl = repl.with_command(Name::command());

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
        Ok(CommandResult::Output(Some(BlockOutput {
            lines: vec![ctx.name.clone()],
        })))
    }
}
