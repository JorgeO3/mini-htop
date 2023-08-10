use clap::Parser;
use std::time::Duration;

pub mod backend;
pub mod system_info;
pub mod ui;
pub mod utils;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long, default_value_t = 250)]
    tick_rate: u64,

    /// Number of times to greet
    #[arg(short, long, default_value_t = true)]
    enhanced_graphics: bool,
}

fn main() {
    let args = Args::parse();

    let tick_rate = Duration::from_millis(args.tick_rate);
}
