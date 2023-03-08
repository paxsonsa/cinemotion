use crate::error::*;
use crate::help::{DefaultHelpViewer, HelpContext, HelpEntry, HelpViewer};
use crate::Value;
use crate::{Command, CommandResult, Parameter};
use rustyline::completion;
use rustyline_derive::{Helper, Highlighter, Hinter, Validator};
use std::boxed::Box;
use std::collections::HashMap;
use std::fmt::Display;
use yansi::Paint;

type ErrorHandler<Context, E> = fn(error: E, repl: &Repl<Context, E>) -> Result<()>;

fn default_error_handler<Context, E: std::fmt::Display>(
    error: E,
    _repl: &Repl<Context, E>,
) -> Result<()> {
    eprintln!("{}", Paint::red(error));
    Ok(())
}

enum ReplState {
    Continue,
    Stop,
}

/// Main REPL struct
pub struct Repl<Context, E: std::fmt::Display> {
    name: String,
    version: String,
    description: String,
    prompt: Box<dyn Display>,
    custom_prompt: bool,
    commands: HashMap<String, Command<Context, E>>,
    context: Context,
    help_context: Option<HelpContext>,
    help_viewer: Box<dyn HelpViewer>,
    error_handler: ErrorHandler<Context, E>,
    use_completion: bool,
}

impl<Context, E> Repl<Context, E>
where
    E: Display + From<Error>,
{
    /// Create a new Repl with the given context's initial value.
    pub fn new(context: Context) -> Self {
        let name = String::new();

        Self {
            name: name.clone(),
            version: String::new(),
            description: String::new(),
            prompt: Box::new(Paint::green(format!("{}> ", name)).bold()),
            custom_prompt: false,
            commands: HashMap::new(),
            context,
            help_context: None,
            help_viewer: Box::new(DefaultHelpViewer::new()),
            error_handler: default_error_handler,
            use_completion: false,
        }
    }

    /// Give your Repl a name. This is used in the help summary for the Repl.
    pub fn with_name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        if !self.custom_prompt {
            self.prompt = Box::new(Paint::green(format!("{}> ", name)).bold());
        }

        self
    }

    /// Give your Repl a version. This is used in the help summary for the Repl.
    pub fn with_version(mut self, version: &str) -> Self {
        self.version = version.to_string();

        self
    }

    /// Give your Repl a description. This is used in the help summary for the Repl.
    pub fn with_description(mut self, description: &str) -> Self {
        self.description = description.to_string();

        self
    }

    /// Give your Repl a custom prompt. The default prompt is the Repl name, followed by
    /// a `>`, all in green, followed by a space.
    pub fn with_prompt(mut self, prompt: &'static dyn Display) -> Self {
        self.prompt = Box::new(prompt);
        self.custom_prompt = true;

        self
    }

    /// Pass in a custom help viewer
    pub fn with_help_viewer<V: 'static + HelpViewer>(mut self, help_viewer: V) -> Self {
        self.help_viewer = Box::new(help_viewer);

        self
    }

    /// Pass in a custom error handler. This is really only for testing - the default
    /// error handler simply prints the error to stderr and then returns
    pub fn with_error_handler(mut self, handler: ErrorHandler<Context, E>) -> Self {
        self.error_handler = handler;

        self
    }

    /// Set whether to use command completion when tab is hit. Defaults to false.
    pub fn use_completion(mut self, value: bool) -> Self {
        self.use_completion = value;

        self
    }

    /// Add a command to your REPL
    pub fn add_command(mut self, command: Command<Context, E>) -> Self {
        self.commands.insert(command.name.clone(), command);

        self
    }

    async fn handle_command(
        &mut self,
        command: &str,
        args: &[&str],
    ) -> core::result::Result<ReplState, E> {
        match self.commands.get_mut(command) {
            Some(definition) => {
                let validated = validate_arguments(&command, &definition.parameters, args)?;
                match (definition.handler.handle(validated, &mut self.context)).await {
                    Ok(result) => match result {
                        CommandResult::Continue(Some(value)) => println!("{}", value),
                        CommandResult::Continue(None) => (),
                        CommandResult::Stop => return Ok(ReplState::Stop),
                    },
                    Err(error) => return Err(error.into()),
                };
            }
            None => {
                if command == "help" {
                    self.show_help(args)?;
                } else {
                    return Err(Error::UnknownCommand(command.to_string()).into());
                }
            }
        }

        Ok(ReplState::Continue)
    }

    fn show_help(&self, args: &[&str]) -> Result<()> {
        if args.is_empty() {
            self.help_viewer
                .help_general(&self.help_context.as_ref().unwrap())?;
        } else {
            let entry_opt = self
                .help_context
                .as_ref()
                .unwrap()
                .help_entries
                .iter()
                .find(|entry| entry.command == args[0]);
            match entry_opt {
                Some(entry) => {
                    self.help_viewer.help_command(&entry)?;
                }
                None => eprintln!("Help not found for command '{}'", args[0]),
            };
        }
        Ok(())
    }

    async fn process_line(&mut self, line: String) -> core::result::Result<ReplState, E> {
        let trimmed = line.trim();
        if !trimmed.is_empty() {
            let r = regex::Regex::new(r#"("[^"\n]+"|[\S]+)"#).unwrap();
            let args = r
                .captures_iter(trimmed)
                .map(|a| a[0].to_string().replace("\"", ""))
                .collect::<Vec<String>>();
            let mut args = args.iter().fold(vec![], |mut state, a| {
                state.push(a.as_str());
                state
            });
            let command: String = args.drain(..1).collect();
            return self.handle_command(&command, &args).await;
        }
        return Ok(ReplState::Continue);
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

    fn create_helper(&mut self) -> Helper {
        let mut helper = Helper::new();
        if self.use_completion {
            for name in self.commands.keys() {
                helper.add_command(name.to_string());
            }
        }

        helper
    }

    pub async fn run(&mut self) -> Result<()> {
        self.construct_help_context();
        let mut editor: rustyline::Editor<Helper> = rustyline::Editor::new();
        let helper = Some(self.create_helper());
        editor.set_helper(helper);
        println!("Welcome to {} {}", self.name, self.version);
        let mut eof = false;
        while !eof {
            self.handle_line(&mut editor, &mut eof).await?;
        }

        Ok(())
    }

    async fn handle_line(
        &mut self,
        editor: &mut rustyline::Editor<Helper>,
        eof: &mut bool,
    ) -> Result<()> {
        match editor.readline(&format!("{}", self.prompt)) {
            Ok(line) => {
                editor.add_history_entry(line.clone());

                match self.process_line(line).await {
                    Ok(ReplState::Continue) => (),
                    Ok(ReplState::Stop) => {
                        *eof = true;
                        return Ok(());
                    }
                    Err(error) => {
                        (self.error_handler)(error, self)?;
                    }
                }
                *eof = false;
                Ok(())
            }
            Err(rustyline::error::ReadlineError::Eof) => {
                *eof = true;
                Ok(())
            }
            Err(error) => {
                eprintln!("Error reading line: {}", error);
                *eof = false;
                Ok(())
            }
        }
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

// rustyline Helper struct
// Currently just does command completion with <tab>, if
// use_completion() is set on the REPL
#[derive(Clone, Helper, Hinter, Highlighter, Validator)]
struct Helper {
    commands: Vec<String>,
}

impl Helper {
    fn new() -> Self {
        Self { commands: vec![] }
    }

    fn add_command(&mut self, command: String) {
        self.commands.push(command);
    }
}

impl completion::Completer for Helper {
    type Candidate = String;

    fn complete(
        &self,
        line: &str,
        _pos: usize,
        _ctx: &rustyline::Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Self::Candidate>)> {
        // Complete based on whether the current line is a substring
        // of one of the set commands
        let ret: Vec<Self::Candidate> = self
            .commands
            .iter()
            .filter(|cmd| cmd.contains(line))
            .map(|s| s.to_string())
            .collect();
        Ok((0, ret))
    }
}
