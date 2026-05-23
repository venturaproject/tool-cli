use clap::{Parser, Subcommand};
use crate::commands::{info::InfoArgs, echo::EchoArgs, json::JsonArgs};

#[derive(Parser)]
#[command(
    name = "tooler",
    version,
    about = "A modular CLI toolkit",
    long_about = None,
    propagate_version = true,
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Show system information
    Info(InfoArgs),

    /// Echo text with optional formatting
    Echo(EchoArgs),

    /// Pretty-print and query JSON
    Json(JsonArgs),
}
