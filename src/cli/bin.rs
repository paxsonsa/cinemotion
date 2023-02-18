use anyhow::{Context, Result};
use clap::Parser;



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
    Ok(0)
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
}

pub fn configure_logging(verbosity: i32, with_timestamps: bool) {
    let base_config = match verbosity {
        n if n <= -3 => String::new(),
        -2 => "indiemotion=error".to_string(),
        -1 => "indiemotion=warn".to_string(),
        0 => std::env::var("INDIEMOTION_LOG")
        .unwrap_or_else(|_| "indiemotion=info".to_string()),
        1 => "indiemotion=debug".to_string(),
        2 => "indiemotion=trace".to_string(),
        _ => "trace".to_string()
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