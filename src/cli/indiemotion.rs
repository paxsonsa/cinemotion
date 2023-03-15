use anyhow::{Context, Result};
use clap::Parser;

mod client;
mod error;
mod server;

use error::{CLIResult, Error};

/// Software development, distribution, and management
#[derive(Parser)]
#[clap(about, author)]
struct Opt {
    /// Make output more verbose
    #[clap(short, long, global = true, parse(from_occurrences))]
    verbose: i32,

    /// Make output less verbose
    ///
    /// This flag takes precedence over the --verbose flag
    #[clap(short, long, global = true, parse(from_occurrences))]
    quiet: i32,

    /// The subcommand to run
    #[clap(subcommand)]
    command: Command,
}

#[derive(clap::Subcommand)]
enum Command {
    Version,
    Server(server::Server),
    Client(client::Client),
}

// FIXME: Add Entity Creation
//        - ECS system for managing objects in the scene.
//        - Add Components to the Entity. When a controller creates a new entity
//          it should be able to add components to it as seperate rpc calls.
// Components:
// - associated with an entity id.
// - components are a discrete set of data types like vectors, floats, integers, or bool
// - an entity can have a number of named components which will be updated each frame.

/*
EntityID - A composite type with a unique id and a name.
EntityComponent - An enum describe the component types.
EntityComponent - A mapping
 */

// TODO: Default Properties
// TODO: Add Property
// TODO: Remove Propertu (except globals)
// TODO: Set Property Mapping
// TODO: TUI Status Line for Client
// TODO: Send Mode Updates
// TODO: Starting Stream Motion
// TODO: Client should be interactive for starting and stopping motion sessions.

impl Command {
    pub fn run(&self) -> Result<i32> {
        let rt = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .context("Failed to establish async runtime")?;
        rt.block_on(self.run_async())
    }

    pub async fn run_async(&self) -> Result<i32> {
        match self {
            Self::Version => {
                println!("indiemotion: {}", indiemotion::VERSION);
                Ok(0)
            }
            Self::Server(cmd) => cmd.run().await,
            Self::Client(cmd) => cmd.run().await,
        }
    }
}

fn main() -> Result<()> {
    let opts = Opt::parse();
    configure_logging(opts.verbose - opts.quiet);
    let code = opts.command.run()?;
    std::process::exit(code);
}

pub fn configure_logging(verbosity: i32) {
    let base_config = match verbosity {
        n if n <= -3 => String::new(),
        -2 => "indiemotion=error".to_string(),
        -1 => "indiemotion=warn".to_string(),
        0 => std::env::var("INDIEMOTION_LOG").unwrap_or_else(|_| "indiemotion=info".to_string()),
        1 => "indiemotion=debug".to_string(),
        2 => "indiemotion=trace".to_string(),
        _ => "trace".to_string(),
    };

    // the RUST_LOG variable will always override the current settings
    let config = match std::env::var("RUST_LOG") {
        Ok(tail) => format!("{},{}", base_config, tail),
        Err(_) => base_config,
    };

    println!("Logging config: {}", config);

    std::env::set_var("INDIEMOTION_LOG", &config);
    // let subscriber = tracing_subscriber::fmt()
    //     // Use a more compact, abbreviated log format
    //     .compact()
    //     .with_target(true)
    //     .without_time()
    //     .finish();
    // use that subscriber to process traces emitted after this point
    // tracing::subscriber::set_global_default(subscriber).unwrap();

    tracing::subscriber::set_global_default(build_logging_subscriber(config)).unwrap();
}

pub fn build_logging_subscriber(
    config: String,
) -> Box<dyn tracing::Subscriber + Send + Sync + 'static> {
    use tracing_subscriber::layer::SubscriberExt;
    let env_filter = tracing_subscriber::filter::EnvFilter::from(config);
    let registry = tracing_subscriber::Registry::default().with(env_filter);
    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_writer(std::io::stderr)
        .with_target(false);
    Box::new(registry.with(fmt_layer.without_time()))
}
