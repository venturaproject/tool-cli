use clap::{Parser, Subcommand};
use crate::{
    commands::{
        check::CheckArgs, completions::CompletionsArgs, config::ConfigArgs, echo::EchoArgs,
        env::EnvArgs, http::HttpArgs, info::InfoArgs, json::JsonArgs, run::RunArgs,
    },
    output::OutputFormat,
};

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

    /// Output format (overrides config default)
    #[arg(long, global = true, value_enum)]
    pub output: Option<OutputFormat>,

    /// Profile to use from config
    #[arg(long, global = true, default_value = "default")]
    pub profile: String,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Show system information
    Info(InfoArgs),

    /// Echo text with optional formatting
    Echo(EchoArgs),

    /// Pretty-print and query JSON
    Json(JsonArgs),

    /// Manage .env files (show, diff, check, get)
    Env(EnvArgs),

    /// Make HTTP requests (GET, POST) with profile auth
    Http(HttpArgs),

    /// Health-check URLs and TCP ports
    Check(CheckArgs),

    /// Manage tooler configuration
    Config(ConfigArgs),

    /// Run a script defined in .tooler.toml
    Run(RunArgs),

    /// Generate shell completion scripts
    Completions(CompletionsArgs),
}
