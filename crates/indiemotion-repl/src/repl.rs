use crate::error::*;
use crate::help::{DefaultHelpViewer, HelpContext, HelpEntry, HelpViewer};
use crate::Result;
use crate::{BlockOutput, Command, CommandOutput, CommandResult, Parameter, Value};
use std::collections::HashMap;
use std::fmt::Display;

pub enum ReplResult {
    Output(CommandOutput),
    Stop,
}

impl ReplResult {
    fn empty() -> Self {
        Self::Output(CommandOutput::Empty)
    }
}

pub struct CommandBlock {
    pub command: String,
    pub output: CommandOutput,
}

impl CommandBlock {
    fn output(command: String, output: CommandOutput) -> Self {
        Self { command, output }
    }

    fn err(command: String, err: String) -> Self {
        Self {
            command,
            output: CommandOutput::Error(BlockOutput {
                lines: vec![format!("Error: {}", err)],
            }),
        }
    }
}

pub struct Repl<Context, E: Display> {
    name: String,
    version: String,
    description: String,
    input_buf: String,
    output: Vec<CommandBlock>,
    context: Context,
    commands: HashMap<String, Command<Context, E>>,
    history: Vec<String>,
    history_index: usize,
    help_context: Option<HelpContext>,
    help_viewer: Box<dyn HelpViewer>,
}

impl<Context, E> Repl<Context, E>
where
    E: Display + From<Error>,
{
    pub fn new(name: String, version: String, description: String, context: Context) -> Self {
        Self {
            name,
            version,
            description,
            input_buf: String::new(),
            output: Vec::new(),
            context,
            commands: HashMap::new(),
            history: Vec::new(),
            history_index: 0,
            help_context: None,
            help_viewer: Box::new(DefaultHelpViewer::new()),
        }
    }

    pub fn current_input(&self) -> &str {
        &self.input_buf
    }

    pub fn push(&mut self, input: char) {
        self.input_buf.push(input);
    }

    pub fn pop(&mut self) {
        self.input_buf.pop();
    }

    pub fn clear_input(&mut self) {
        self.input_buf.clear();
        self.history_index = self.history.len();
    }

    pub fn history_up(&mut self) {
        if self.history.is_empty() {
            return;
        }

        if self.history_index == 0 {
            self.input_buf = self.history[self.history_index].clone();
            return;
        }
        self.history_index = self.history_index - 1;
        self.input_buf = self.history[self.history_index].clone();
    }

    pub fn history_down(&mut self) {
        let index = self.history_index + 1;
        if index >= self.history.len() {
            self.history_index = self.history.len();
            self.input_buf.clear();
            return;
        }

        self.history_index = index;
        self.input_buf = self.history[self.history_index].clone();
    }

    pub async fn process_input(&mut self) -> Result<()> {
        if self.help_context.is_none() {
            self.construct_help_context();
        }

        let trimmed = self.input_buf.trim().to_string();
        self.history.push(trimmed.to_string());
        self.clear_input();

        if !trimmed.is_empty() {
            let r = regex::Regex::new(r#"("[^"\n]+"|[\S]+)"#).unwrap();
            let args = r
                .captures_iter(&trimmed)
                .map(|a| a[0].to_string().replace("\"", ""))
                .collect::<Vec<String>>();
            let mut args = args.iter().fold(vec![], |mut state, a| {
                state.push(a.as_str());
                state
            });
            let command: String = args.drain(..1).collect();

            match self.handle_command(&command, &args).await {
                Ok(ReplResult::Output(block)) => {
                    self.output.push(CommandBlock::output(trimmed, block))
                }
                Ok(ReplResult::Stop) => return Err(Error::Stopped),
                Err(err) => self
                    .output
                    .push(CommandBlock::err(trimmed, err.to_string())),
            }
        }
        Ok(())
    }

    pub fn with_command(mut self, command: Command<Context, E>) -> Self {
        self.commands.insert(command.name.clone(), command);
        self
    }

    pub fn output(&self) -> &Vec<CommandBlock> {
        &self.output
    }

    pub async fn handle_command(
        &mut self,
        command: &str,
        args: &[&str],
    ) -> core::result::Result<ReplResult, E> {
        match self.commands.get_mut(command) {
            Some(definition) => {
                let validated = validate_arguments(&command, &definition.parameters, args)?;
                match (definition.handler.handle(validated, &mut self.context)).await {
                    Ok(result) => match result {
                        CommandResult::Output(output) => Ok(ReplResult::Output(output)),
                        CommandResult::Stop => Ok(ReplResult::Stop),
                    },
                    Err(error) => Ok(ReplResult::Output(CommandOutput::Error(BlockOutput {
                        lines: vec![format!("Error: {}", error)],
                    }))),
                }
            }
            None => match command {
                "help" => Ok(ReplResult::Output(CommandOutput::Info(
                    self.show_help(args)?,
                ))),
                "clear" => {
                    self.output.clear();
                    Ok(ReplResult::empty())
                }
                _ => Err(Error::UnknownCommand(command.to_string()).into()),
            },
        }
    }

    fn show_help(&self, args: &[&str]) -> Result<BlockOutput> {
        if args.is_empty() {
            return Ok(self
                .help_viewer
                .help_general(&self.help_context.as_ref().unwrap())?);
        } else {
            let entry_opt = self
                .help_context
                .as_ref()
                .unwrap()
                .help_entries
                .iter()
                .find(|entry| entry.command == args[0]);
            return match entry_opt {
                Some(entry) => Ok(self.help_viewer.help_command(&entry)?),
                None => Ok(BlockOutput::error(format!(
                    "Help not found for command '{}'",
                    args[0]
                ))),
            };
        }
    }

    fn construct_help_context(&mut self) {
        let mut help_entries = self
            .commands
            .iter()
            .map(|(_, definition)| {
                HelpEntry::new(
                    &definition.name,
                    &definition.parameters,
                    &definition.help_summary,
                )
            })
            .collect::<Vec<HelpEntry>>();
        help_entries.sort_by_key(|d| d.command.clone());
        self.help_context = Some(HelpContext::new(
            &self.name,
            &self.version,
            &self.description,
            help_entries,
        ));
    }
}

fn validate_arguments(
    command: &str,
    parameters: &[Parameter],
    args: &[&str],
) -> Result<HashMap<String, Value>> {
    if args.len() > parameters.len() {
        return Err(Error::TooManyArguments(command.into(), parameters.len()));
    }

    let mut validated = HashMap::new();
    for (index, parameter) in parameters.iter().enumerate() {
        if index < args.len() {
            validated.insert(parameter.name.clone(), Value::new(&args[index]));
        } else if parameter.required {
            return Err(Error::MissingRequiredArgument(
                command.into(),
                parameter.name.clone(),
            ));
        } else if parameter.default.is_some() {
            validated.insert(
                parameter.name.clone(),
                Value::new(&parameter.default.clone().unwrap()),
            );
        }
    }
    Ok(validated)
}

// pub(crate) struct Role;

// #[async_trait::async_trait]
// impl indiemotion_repl::CommandHandler for Role {
//     type Context = context::Context;
//     type Error = Error;

//     async fn handle(
//         &mut self,
//         args: HashMap<String, Value>,
//         ctx: &mut Self::Context,
//     ) -> std::result::Result<indiemotion_repl::CommandResult, Error> {
//         if let Some(role) = args.get("role") {
//             let s: String = role.convert()?;
//             ctx.role = s.try_into()?;
//         }
//         Ok(CommandResult::Continue(Some(ctx.role.clone().into())))
//     }
// }

// pub(crate) struct Ping;

// #[async_trait::async_trait]
// impl indiemotion_repl::CommandHandler for Ping {
//     type Context = context::Context;
//     type Error = Error;

//     async fn handle(
//         &mut self,
//         _args: HashMap<String, Value>,
//         ctx: &mut Self::Context,
//     ) -> std::result::Result<indiemotion_repl::CommandResult, Error> {
//         check_connection(ctx)?;

//         let timestamp = chrono::Utc::now().timestamp_millis();

//         let request = proto::PingRequest { timestamp };
//         match ctx.client.as_mut().unwrap().ping(request).await {
//             Ok(response) => {
//                 // let timestamp = chrono::Utc::now().timestamp_millis();
//                 let response = response.into_inner();
//                 let timestamp = response.client_timestamp;
//                 let server_ts = response.server_timestamp;
//                 let runtime_ts = response.runtime_timestamp;
//                 Ok(CommandResult::Continue(Some(format!(
//                     "server: {}ms   runtime: {}ms    roundtrip: {}ms",
//                     (server_ts - timestamp),
//                     (runtime_ts - timestamp),
//                     (runtime_ts - timestamp) * 2
//                 ))))
//             }
//             Err(err) => {
//                 tracing::error!("Failed to ping: {}", err);
//                 Err(Error::CommandFailed(format!("failed to ping: {:?}", err)))
//             }
//         }
//     }
// }

// pub(crate) struct Connect;

// #[async_trait::async_trait]
// impl indiemotion_repl::CommandHandler for Connect {
//     type Context = context::Context;
//     type Error = Error;

//     async fn handle(
//         &mut self,
//         _args: HashMap<String, Value>,
//         ctx: &mut Self::Context,
//     ) -> std::result::Result<indiemotion_repl::CommandResult, Error> {
//         ctx.connect().await?;

//         let info = proto::ClientInfo {
//             id: ctx.uid.clone().to_string(),
//             name: ctx.name.clone(),
//             role: Into::<proto::ClientRole>::into(ctx.role.clone()).into(),
//         };

//         let req = proto::ConnectAsRequest {
//             client_info: Some(info),
//         };

//         println!("Connecting to server...");
//         match ctx.client.as_mut().unwrap().connect_as(req).await {
//             Ok(response) => {
//                 println!("Establishing loop...");
//                 let mut stream = response.into_inner();
//                 ctx.main_loop = Some(tokio::spawn(async move {
//                     while let Some(event) = stream.message().await.unwrap() {
//                         println!("Event: {:?}", event);
//                     }
//                 }));
//                 Ok(CommandResult::Continue(Some("connected".to_string())))
//             }
//             Err(_) => Ok(CommandResult::Continue(Some(
//                 "failed to connect".to_string(),
//             ))),
//         }
//     }
// }
