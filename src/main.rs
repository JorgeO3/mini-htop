use clap::Parser;
use prelude::*;
use std::time::Duration;

pub mod backend;
pub mod error;
pub mod prelude;
pub mod system_info;
pub mod ui;
pub mod utils;

/// Minitop: A User-Friendly Top Monitor for Process Viewing
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// time in ms between two ticks.
    #[arg(short, long, default_value_t = 250)]
    tick_rate: u64,

    /// whether unicode symbols are used to improve the overall look of the app
    #[arg(short, long, default_value_t = true)]
    enhanced_graphics: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let tick_rate = Duration::from_millis(cli.tick_rate);
    crate::backend::run(tick_rate, cli.enhanced_graphics)?;

    Ok(())
}
