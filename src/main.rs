use clap::Parser;
use prelude::*;
use std::time::Duration;

pub mod backend;
pub mod error;
pub mod prelude;
pub mod system_info;
pub mod ui;
pub mod utils;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Name of the person to greet
    #[arg(short, long, default_value_t = 250)]
    tick_rate: u64,

    /// Number of times to greet
    #[arg(short, long, default_value_t = true)]
    enhanced_graphics: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let tick_rate = Duration::from_millis(cli.tick_rate);
    crate::backend::run(tick_rate, cli.enhanced_graphics)?;

    Ok(())
}
