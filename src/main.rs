mod cli;
mod commands;
mod error;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Info(args) => commands::info::run(args),
        Commands::Echo(args) => commands::echo::run(args),
        Commands::Json(args) => commands::json::run(args),
    }
}
