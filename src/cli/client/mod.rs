use anyhow::Result;
use clap::Args;

use std::fmt::Display;
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
    address: Option<String>,
}

impl Client {
    pub async fn run(&self) -> Result<i32> {
        Ok(0)
    }
}
