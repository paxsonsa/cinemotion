use anyhow::{Context, Result};
use clap::Parser;
use tracing_subscriber::Layer;

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

/* TODO: TUI for Client

    - TODO: Add Log View
        - Create log view that displays log events from a custom tracing layer
        - Create a log field visitor so we can access and display log values.
                ref: https://burgers.io/custom-logging-in-rust-using-tracing

    - Add Info Command
    - Add Connect Command
    - Add Top Status Line for Connection Status
    - Add Ping Command

*/
/* TODO Entity Resources
- Add Entity.
- Remove Entity.
- Add Component to entity.
- Remove Component from entity.
- Add Entity Client Map.
- Remove Entity Client Map.

// TODO - Scenes.

 */
// TODO: Send Mode Updates
// TODO: Starting Stream Motion
// TODO: Client should be interactive for starting and stopping motion sessions.

impl Command {
    pub fn configure_logging(&self, verbosity: i32) {
        let base_config = match verbosity {
            n if n <= -3 => String::new(),
            -2 => "indiemotion=error".to_string(),
            -1 => "indiemotion=warn".to_string(),
            0 => {
                std::env::var("INDIEMOTION_LOG").unwrap_or_else(|_| "indiemotion=info".to_string())
            }
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
        use tracing_subscriber::layer::SubscriberExt;
        let env_filter = tracing_subscriber::filter::EnvFilter::from(config);
        let registry = tracing_subscriber::Registry::default().with(env_filter);

        let layer = match self {
            Self::Version => None,
            Self::Server(_cmd) => None,
            Self::Client(cmd) => Some(cmd.logging_layer()),
        }
        .unwrap_or_else(|| {
            Box::new(
                tracing_subscriber::fmt::layer()
                    .with_writer(std::io::stderr)
                    .with_target(false)
                    .without_time(),
            )
        });

        tracing::subscriber::set_global_default(registry.with(layer)).unwrap();
    }

    pub fn run(&self, opts: &Opt) -> Result<i32> {
        self.configure_logging(opts.verbose - opts.quiet);

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
    // configure_logging(opts.verbose - opts.quiet);
    let code = opts.command.run(&opts)?;
    std::process::exit(code);
}
