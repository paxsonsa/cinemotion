use anyhow::{Context, Result};
use clap::{arg, Args, Parser};
use indiemotion::ServerBuilder;
use tokio::signal::unix::{signal, SignalKind};

/**
 * TODO
 * - Create sucommands for server and client
 * - Client should be interactive for starting and stopping motion sessions.
 * - Add Properties to the server
 * - Remove Properties
 * - Send State Updates
 * - Starting Stream Motion
 */

fn main() -> Result<()> {
    let opt = Opt::parse();
    configure_logging(opt.verbose, opt.log_time);
    // args::configure_logging(opts.verbose - opts.quiet, opts.log_time);
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .context("Failed to establish async runtime")?;
    let code = rt.block_on(run(opt))?;
    std::process::exit(code);
}

async fn run(_option: Opt) -> Result<i32> {
    let server_builder = _option.server;
    let mut server = server_builder.build().await?;

    let (shutdown_send, shutdown) = tokio::sync::oneshot::channel();
    let server_future = server.serve_with_shutdown(async move {
        let _ = shutdown.await;
    });
    tracing::debug!("server future is ready");

    let mut sigterm = signal(SignalKind::terminate())?;
    let interrupt_task = tokio::task::spawn(async move {
        tracing::debug!("listening for interrupt signals...");
        tokio::select! {
            _ = sigterm.recv() => {
                tracing::info!("terminate recevied, shutting down...");
            },
            _ = tokio::signal::ctrl_c() => {
                tracing::info!("interrupt recevied, shutting down...");
            },
        };
        let _ = shutdown_send.send(());
    });

    tracing::info!("ready");
    if let Err(err) = server_future.await {
        tracing::error!(?err, "Server encountered and error");
        interrupt_task.abort();
        Ok(1)
    } else {
        interrupt_task.abort();
        Ok(0)
    }
}

#[derive(Debug, Parser)]
struct Opt {
    /// Make output more verbose
    #[clap(short, long, global = true, parse(from_occurrences))]
    verbose: i32,

    /// Make output less verbose
    ///
    /// This flag takes precedence over the --verbose flag
    #[clap(short, long, global = true, parse(from_occurrences))]
    quiet: i32,

    /// Add timestamps to any log output
    #[clap(long, global = true)]
    log_time: bool,

    #[clap(flatten)]
    server: indiemotion::ServerBuilder,
}

pub fn configure_logging(verbosity: i32, with_timestamps: bool) {
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

    std::env::set_var("INDIEMOTION_LOG", &config);

    tracing::subscriber::set_global_default(build_logging_subscriber(config, with_timestamps))
        .unwrap();
}

pub fn build_logging_subscriber(
    config: String,
    with_timestamps: bool,
) -> Box<dyn tracing::Subscriber + Send + Sync + 'static> {
    use tracing_subscriber::layer::SubscriberExt;
    let env_filter = tracing_subscriber::filter::EnvFilter::from(config);
    let registry = tracing_subscriber::Registry::default().with(env_filter);
    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_writer(std::io::stderr)
        .with_target(false);
    if with_timestamps {
        Box::new(registry.with(fmt_layer))
    } else {
        Box::new(registry.with(fmt_layer.without_time()))
    }
}
